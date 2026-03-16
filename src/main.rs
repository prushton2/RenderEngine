use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, RwLock};

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::object::Renderable;

mod object;
mod ds;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    context: Option<Context<Rc<Window>>>,

    player: Arc<RwLock<object::Player>>,
    objects: Arc<Vec<Box<dyn object::Renderable + Send + Sync>>>,
    
    keyboard: HashMap<KeyCode, bool>,
    last_frame: std::time::Instant,
    deltatime: f64,

    // statistics
    fps_over_last_second: Vec<f64>,
    deltatime_over_last_second: Vec<f64>,
    average_fps: u32,
    average_deltatime: u32,
    statistics_timer: std::time::Instant
}

impl App {
    pub fn consume_player(&mut self, player: object::Player) {
        self.player = Arc::new(RwLock::new(player));
    }

    pub fn consume_objects(&mut self, objects: Vec<Box<dyn object::Renderable + Send + Sync>>) {
        self.objects = Arc::new(objects);
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
                player_ref.move_player(&(dir * self.deltatime));
            }
        }

        for (key, dir) in key_rotations {
            if self.keyboard.get(key) == Some(&true) {
                player_ref.change_rotation(dir * self.deltatime);
            }
        }

        player_ref.update_outputs();
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            surface: None,
            context: None,

            player: Arc::new(RwLock::new(object::Player::new(object::Camera::zero()))),
            objects: vec![].into(),
            keyboard: [].into(),
            last_frame: std::time::Instant::now(),
            deltatime: 0.0,

            fps_over_last_second: vec![],
            deltatime_over_last_second: vec![],
            average_fps: 0,
            average_deltatime: 0,
            statistics_timer: std::time::Instant::now()
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Render Engine")
                        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64)),
                )
                .expect("Failed to create window"),
        );

        let context = Context::new(window.clone()).expect("Failed to create softbuffer context");
        let surface = Surface::new(&context, window.clone()).expect("Failed to create surface");

        self.context = Some(context);
        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
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

                if self.surface.is_none() || self.window.is_none() {
                    return;
                }
                                
                let window = self.window.as_ref().unwrap();
                let size = window.inner_size();
                
                let width_nzu32 = match NonZeroU32::new(size.width) {
                    Some(t) => t,
                    None => return
                };
                
                let height_nzu32 = match NonZeroU32::new(size.height) {
                    Some(t) => t,
                    None => return

                };

                let width: usize = size.width as usize;
                let height: usize = size.height as usize;
                
                self.player.write().unwrap().get_camera_mut().set_window_size(size.width.into(), size.height.into());
                self.handle_movement();

                
                let surface = self.surface.as_mut().unwrap();
                surface.resize(width_nzu32, height_nzu32).expect("Failed to resize surface");
                
                let mut buf = surface.buffer_mut().expect("Failed to get buffer");
                
                let thread_count = 8;
                let mut threads = vec![];

                for i in 0..thread_count {
                    let player_ref = Arc::clone(&self.player);
                    let objects_ref = Arc::clone(&self.objects);

                    threads.push(thread::spawn(move || {
                        let strip_start = (height / thread_count) * i;
                        let strip_end = if i == thread_count - 1 {
                            height  // last thread takes any leftover rows
                        } else {
                            (height / thread_count) * (i + 1)
                        };
                        let strip_height = strip_end - strip_start;

                        let mut pixels: Vec<u32> = vec![0; width * strip_height];
                        let player_read = player_ref.read().unwrap();

                        for x in 0..width {
                            for y in strip_start..strip_end {
                                pixels[(y - strip_start) * width + x] = player_read.get_camera().get_pixel_color(objects_ref.as_ref(), x as f64, y as f64);
                            }
                        }

                        pixels
                    }));
                }

                let mut pixels: Vec<u32> = Vec::with_capacity(width*height);

                for thread in threads.drain(0..threads.len()) {
                    pixels.extend(thread.join().unwrap());
                }

                buf.copy_from_slice(&pixels);

                buf.present().expect("Failed to present buffer");

                let player_mut = self.player.read().unwrap();

                if self.statistics_timer.elapsed().as_millis() >= 1000 {
                    self.average_fps = {
                        let mut sum = 0.0;
                        for i in &self.fps_over_last_second {
                            sum += i;
                        }
                        sum/self.fps_over_last_second.len() as f64
                    } as u32;

                    self.average_deltatime = {
                        let mut sum = 0.0;
                        for i in &self.deltatime_over_last_second {
                            sum += i;
                        }
                        sum/self.deltatime_over_last_second.len() as f64
                    } as u32;

                    self.fps_over_last_second.clear();
                    self.deltatime_over_last_second.clear();
                    self.statistics_timer = std::time::Instant::now();
                }

                self.fps_over_last_second.push(1.0/self.deltatime);
                self.deltatime_over_last_second.push(self.deltatime*1000.0);

                print!("\x1B[2J\x1B[1;1H");
                println!(" FPS: {}\n\n Time between frames: {}ms\n\n Camera position: {:?}\n Player Rotation: {:?}", self.average_fps, self.average_deltatime, player_mut.get_camera().pos(), player_mut.get_rotation());
            }

            WindowEvent::Resized(_) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
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
    let camera = object::Camera::new(
        ds::Vector3::new(0.0, 0.0, 0.0),
        3.0,
        (WIDTH as f64, HEIGHT as f64),
        60.0
    );

    let player = object::Player::new(
        camera
    );

    let objects: Vec<Box<dyn object::Renderable + Send + Sync>> = vec![
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 0.0, 7.0), 0.1)),
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 0.0, 5.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(-2.0, -0.4, 5.0), 0.1)),

        Box::new(object::Quad::new(&ds::Vector3::new(-1.0, -1.0, 6.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0))),

        Box::new(object::Sphere::new(&ds::Vector3::new(-3.25, -0.8, 6.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.75, -0.8, 6.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   0.0, 6.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   1.0, 6.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   2.0, 6.0), 0.5))
    ];

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    app.consume_player(player);
    app.consume_objects(objects);

    event_loop.run_app(&mut app).expect("Event loop failed");
}

