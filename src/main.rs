use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use clap::Parser;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent, DeviceEvent, DeviceId};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId, CursorGrabMode};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::material::GpuMaterial;
use crate::object::Renderable;
use crate::object::renderable::ToGpu;

mod wgpu_handler;
mod material;
mod object;
mod ds;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = String::from("1280x720"))]
    pub resolution: String,

    #[arg(short, long, default_value_t = 0.001)]
    pub sensitivity: f64,

    #[arg(short, long, default_value_t = 144)]
    pub framelimit: u64,

    #[arg(short, long, default_value_t = 1.5)]
    pub movespeed: f64,

    #[arg(default_value_t = 1920)]
    pub width: usize,
    #[arg(default_value_t = 1080)]
    pub height: usize,
}

impl Arguments {
    fn update(&mut self) {
        let mut split = self.resolution.split('x');
        let res_err = "Invalid resolution";
        (self.width, self.height) = (
            split.next().expect(res_err).parse().expect(res_err),
            split.next().expect(res_err).parse().expect(res_err)
        );

        if self.framelimit > 1000 {
            self.framelimit = 1000;
        }
    }
}

struct App {
    // window
    window: Option<Arc<Window>>,
    gpu:    wgpu_handler::GpuHandler,

    // scene
    player:  RwLock<object::Player>,
    objects: Vec<Box<dyn object::Renderable>>,

    // input
    keyboard:     HashMap<KeyCode, bool>,
    mouse_delta: (f64, f64),
    config:       Arguments,

    // statistics
    last_frame: std::time::Instant,
    deltatime:  f64,

    fps_stat:         u32,
    deltatime_stat:   u32,
    statistics_timer: std::time::Instant,
}

impl App {
    pub fn new(config: Arguments, player: object::Player, objects: Vec<Box<dyn object::Renderable>>) -> Self {
        Self {
            window: None,
            gpu:    wgpu_handler::GpuHandler::default(),

            player:  RwLock::new(player),
            objects: objects,

            keyboard:     HashMap::new(),
            mouse_delta: (0.0, 0.0),
            config:       config,

            last_frame: std::time::Instant::now(),
            deltatime:  0.0,

            fps_stat:         0,
            deltatime_stat:   0,
            statistics_timer: std::time::Instant::now(),
        }
    }

    pub fn handle_movement(&mut self) {
        let mut player_ref = self.player.write().unwrap();
        let key_movements: &[(KeyCode, ds::Vector3)] = &[
            (KeyCode::KeyW,        ds::Vector3::new( 0.0,  0.0,  1.0)),
            (KeyCode::KeyS,        ds::Vector3::new( 0.0,  0.0, -1.0)),
            (KeyCode::KeyA,        ds::Vector3::new(-1.0,  0.0,  0.0)),
            (KeyCode::KeyD,        ds::Vector3::new( 1.0,  0.0,  0.0)),
            (KeyCode::Space,       ds::Vector3::new( 0.0,  1.0,  0.0)),
            (KeyCode::ControlLeft, ds::Vector3::new( 0.0, -1.0,  0.0)),
        ];

        let key_rotations: &[(KeyCode, ds::Vector3)] = &[
            (KeyCode::ArrowLeft,  ds::Vector3::new( 0.0,  0.0, -0.5)),
            (KeyCode::ArrowRight, ds::Vector3::new( 0.0,  0.0,  0.5)),
            (KeyCode::ArrowUp,    ds::Vector3::new( 0.5,  0.0,  0.0)),
            (KeyCode::ArrowDown,  ds::Vector3::new(-0.5,  0.0,  0.0)),
        ];

        for (key, dir) in key_movements {
            if self.keyboard.get(key) == Some(&true) {
                player_ref.move_player(&(dir * self.config.movespeed * self.deltatime));
            }
        }

        for (key, dir) in key_rotations {
            if self.keyboard.get(key) == Some(&true) {
                player_ref.change_rotation(dir * self.deltatime);
            }
        }

        player_ref.change_rotation(ds::Vector3::new(-self.mouse_delta.1, 0.0, self.mouse_delta.0));
        self.mouse_delta = (0.0, 0.0);

        player_ref.update_outputs();
    }

    pub fn render(&self) -> Option<wgpu::SurfaceTexture> {

        let player = self.player.read().unwrap();

        // downcast objects
        let mut uniform = player.get_camera().to_gpu();

        let gpu_spheres: Vec<object::sphere::GpuSphere> = self.objects.iter()
            .filter_map(|o| o.as_any().downcast_ref::<object::Sphere>())
            .map(|s| s.to_gpu())
            .collect();
        
        let gpu_quads: Vec<object::quad::GpuQuad> = self.objects.iter()
            .filter_map(|o| o.as_any().downcast_ref::<object::Quad>())
            .map(|s| s.to_gpu())
            .collect();
        
        return self.gpu.draw_frame(&gpu_spheres, &gpu_quads, &mut uniform);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(
                Window::default_attributes()
                    .with_title("Render Engine")
                    .with_inner_size(winit::dpi::LogicalSize::new(self.config.width as f64, self.config.height as f64))
            ).unwrap()
        );

        window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();
        window.set_cursor_visible(false);

        // wgpu init is async but resumed() isn't — use pollster to block
        pollster::block_on(
            self.gpu.init(
                window.clone(), 
                self.config.width as u32, 
                self.config.height as u32, 
            vec!["textures/dirt.png", "textures/grass_side.png", "textures/grass_top.png"])
        );

        std::thread::sleep(std::time::Duration::from_millis(1000));

        self.window = Some(window);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.mouse_delta = (self.mouse_delta.0 + (dx as f64)*self.config.sensitivity, self.mouse_delta.1 + (dy as f64)*self.config.sensitivity);
            },
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.deltatime = self.last_frame.elapsed().as_millis() as f64 / 1000.0;
                self.last_frame = std::time::Instant::now();
                
                if self.window.is_none() {
                    println!("No Window");
                    return;
                }
                
                let window = self.window.as_ref().unwrap();
                let size = window.inner_size();
                
                self.player.write().unwrap().get_camera_mut().set_window_size(size.width.into(), size.height.into());
                self.handle_movement();
                
                if let Some(frame) = self.render() {
                    frame.present();
                }
                
                let player = self.player.read().unwrap();
                
                if self.statistics_timer.elapsed().as_millis() >= 1000 {
                    self.fps_stat = (1.0/self.deltatime) as u32;
                    self.deltatime_stat = (1000.0 * self.deltatime) as u32;
                    self.statistics_timer = std::time::Instant::now();
                }
                
                print!("\x1B[2J\x1B[1;1H");
                println!(" FPS: {}\n\n Time between frames: {}ms\n\n Camera position: {:?}\n Player Rotation: {:?}", self.fps_stat, self.deltatime_stat, player.get_camera().pos(), player.get_rotation());

                // this makes the deltatime not crash out when the fps gets too high,
                // but caps the fps at 1000
                if self.deltatime < 1000.0/self.config.framelimit as f64 {
                    let mut duration = std::time::Duration::from_millis(1000/self.config.framelimit);
                    duration -= std::time::Duration::from_millis(self.deltatime as u64);
                    std::thread::sleep(duration);
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::Resized(new_size) => {
                let window = match &self.window {
                    Some(t) => t,
                    None => return,
                };

                self.gpu.change_resolution(new_size.width.into(), new_size.height.into());
                
                window.request_redraw();
            }

            WindowEvent::KeyboardInput{
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(keycode),
                    state,
                    ..
                },
                ..
            } => {
                match (keycode, state) {
                    // special functions
                    (KeyCode::Escape, ElementState::Pressed) => event_loop.exit(),
                    (keycode, pressed) => {
                        // everything else is mapped to the keyboard hashmap
                        self.keyboard.insert(keycode, pressed == ElementState::Pressed);
                    }
                }
            }

            _ => {}
        }
    }
}

fn main() {
    debug_assert_eq!(std::mem::size_of::<object::camera::GpuUniform>() % 256, 0);

    let mut args = Arguments::parse();
    args.update();

    let camera = object::Camera::new(
        ds::Vector3::new(0.0, 0.0, 0.0),
        3.0,
        (args.width as f64, args.height as f64),
        60.0
    );

    let player = object::Player::new(
        camera
    );

    let objects: Vec<Box<dyn Renderable>> = vec![


        // three spheres
        Box::new(object::Sphere::new(&ds::Vector3::new( 0.0,  0.75, 5.0), 0.25, GpuMaterial::new(0x00FF0000, 0, 0))),
        Box::new(object::Sphere::new(&ds::Vector3::new( 0.0,  0.0,  5.0), 0.25, GpuMaterial::new(0x0000FF00, 0, 0))),
        Box::new(object::Sphere::new(&ds::Vector3::new( 0.0, -0.75, 5.0), 0.25, GpuMaterial::new(0x000000FF, 0, 0))),

        // reflective and translucent pane
        Box::new(object::Quad::new(&ds::Vector3::new( -1.0,  -1.0, 4.5), &ds::Vector3::new(0.0, 0.0, 1.0), &ds::Vector3::new(0.0, 2.0, 0.0), GpuMaterial::new(0x00888888, 33, 33))),

        // colored windows
        Box::new(object::Quad::new(&ds::Vector3::new(-2.0, -0.5, 1.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::new(0x00FF0000, 0, 50))),
        Box::new(object::Quad::new(&ds::Vector3::new(-3.0, -0.5, 1.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::new(0x0000FF00, 0, 50))),
        Box::new(object::Quad::new(&ds::Vector3::new(-4.0, -0.5, 1.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::new(0x000000FF, 0, 50))),

        // colored mirrors
        Box::new(object::Quad::new(&ds::Vector3::new(-2.0, 1.5, 10.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, -0.5), GpuMaterial::new(0x00FF0000, 50, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(-3.0, 1.5, 10.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, -0.5), GpuMaterial::new(0x0000FF00, 50, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(-4.0, 1.5, 10.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, -0.5), GpuMaterial::new(0x000000FF, 50, 0))),

        // double mirror
        Box::new(object::Quad::new(&ds::Vector3::new(-5.0, -0.5, 3.0), &ds::Vector3::new(0.0, 0.0, 1.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::new(0x00888888, 75, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(-7.0, -0.5, 3.0), &ds::Vector3::new(0.0, 0.0, 1.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::new(0x00888888, 75, 0))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-6.0, 0.0, 3.5), 0.1, GpuMaterial::new(0x0000FF, 0, 0))),

        //
        Box::new(object::Sphere::new(&ds::Vector3::new(-3.25, -0.8, 6.0), 0.5,  GpuMaterial::new(0x0000FF00,  0,  0))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.75, -0.8, 6.0), 0.5,  GpuMaterial::new(0x000000FF,  0,  0))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   0.0, 6.0), 0.5,  GpuMaterial::new(0x00FF69B4, 50,  0))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   1.0, 6.0), 0.49, GpuMaterial::new(0x00FF69B4,  0, 50))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   2.0, 6.0), 0.49, GpuMaterial::new(0x00FF0000,  0,  0))),

        // mirror sphere
        Box::new(object::Sphere::new(&ds::Vector3::new(4.0,   0.0, 3.0), 2.0,  GpuMaterial::new(0x00AAAAAA, 90,  0))),
        Box::new(object::Sphere::new(&ds::Vector3::new(4.0,   0.3, 3.0), 0.25, GpuMaterial::new(0x000000FF,  0,  0))),

        // grass block
        Box::new(object::Quad::new(&ds::Vector3::new(1.0, 0.0, 5.0), &ds::Vector3::new(0.0, 0.0, 1.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::texture(1, 0, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(1.0, 0.0, 6.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::texture(1, 0, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(2.0, 0.0, 6.0), &ds::Vector3::new(0.0, 0.0,-1.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::texture(1, 0, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(2.0, 0.0, 5.0), &ds::Vector3::new(-1.0,0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), GpuMaterial::texture(1, 0, 0))),

        Box::new(object::Quad::new(&ds::Vector3::new(1.0, 1.0, 5.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 0.0, 1.0), GpuMaterial::texture(2, 0, 0))),
        Box::new(object::Quad::new(&ds::Vector3::new(1.0, 0.0, 5.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 0.0, 1.0), GpuMaterial::texture(0, 0, 0))),

        Box::new(object::Quad::new(&ds::Vector3::new(10.0, 2.0, 7.0), &ds::Vector3::new(0.5, 1.0, 0.5), &ds::Vector3::new(1.0, 0.5, 0.0), GpuMaterial::texture(0, 0, 0))),

    ];

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(args, player, objects);

    event_loop.run_app(&mut app).expect("Event loop failed");
}