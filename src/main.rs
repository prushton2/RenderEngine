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
            (KeyCode::KeyW,      ds::Vector3::new( 0.0,  0.0,  1.0)),
            (KeyCode::KeyS,      ds::Vector3::new( 0.0,  0.0, -1.0)),
            (KeyCode::KeyA,      ds::Vector3::new(-1.0,  0.0,  0.0)),
            (KeyCode::KeyD,      ds::Vector3::new( 1.0,  0.0,  0.0)),
            (KeyCode::Space,     ds::Vector3::new( 0.0,  1.0,  0.0)),
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
            deltatime: 0.0
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
                
                self.handle_movement();
                
                let window = self.window.as_ref().unwrap();
                let surface = self.surface.as_mut().unwrap();

                let size = window.inner_size();
                let width = size.width;
                let height = size.height;

                let w = match NonZeroU32::new(width) {
                    Some(t) => t,
                    None => return
                };

                let h = match NonZeroU32::new(height) {
                    Some(t) => t,
                    None => return
                };

                surface.resize(w, h).expect("Failed to resize surface");
                
                let mut buf = surface.buffer_mut().expect("Failed to get buffer");
                
                let thread_count = 8;
                let mut threads = vec![];

                for i in 0..thread_count {
                    let player_ref = Arc::clone(&self.player);
                    let objects_ref = Arc::clone(&self.objects);

                    threads.push(thread::spawn(move || {
                        let mut pixels: Vec<u32> = vec![0; WIDTH * HEIGHT/thread_count];
                        let player_read = player_ref.read().unwrap();

                        for x in 0..WIDTH {
                            for y in ((HEIGHT/thread_count)*i)..(HEIGHT/thread_count)*(i+1) {
                                pixels[(y - (HEIGHT/thread_count)*i) * WIDTH + x] = get_pixel_color(player_read.get_camera(), objects_ref.as_ref(), x as f64, y as f64)
                            }
                        }

                        pixels
                    }));
                }

                let mut pixels: Vec<u32> = Vec::with_capacity(WIDTH*HEIGHT);

                for thread in threads.drain(0..threads.len()) {
                    pixels.extend(thread.join().unwrap());
                }

                buf.copy_from_slice(&pixels);

                buf.present().expect("Failed to present buffer");

                let player_mut = self.player.read().unwrap();

                print!("\x1B[2J\x1B[1;1H");
                println!(" FPS: {}\n\n Time between frames: {}ms\n\n Camera position: {:?}\n Player Rotation: {:?}", (1.0/self.deltatime) as u32, self.deltatime*1000.0, player_mut.get_camera().pos(), player_mut.get_rotation());
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

fn get_pixel_color(camera: &object::Camera, objects: &Vec<Box<dyn object::Renderable + Send + Sync>>, x: f64, y: f64) -> u32 {
    let pixel_center = camera.pixel00_loc() + (x * camera.pixel_delta_w()) + (y * camera.pixel_delta_h());
    let ray_direction = pixel_center - camera.pos();
    let ray = ds::Ray::new(&camera.pos(), &ray_direction);

    let mut lowest_distance: Option<f64> = None;
    let mut color = 0x0087CEEB;

    for object in objects {
        let intersects = object.intersects(&ray);
        
        // we dont intersect or its behind the camera
        if intersects.is_none() || intersects.unwrap() < 0.0 {
            continue
        }

        let t = intersects.unwrap();
        let surface_pos = ray.at(t);
        let len_sq = (surface_pos - camera.pos()).length_sq();

        if lowest_distance == None || len_sq < lowest_distance.unwrap() {
            lowest_distance = Some(len_sq);
            color = object.color(&surface_pos);
        }
    }
    
    return color;
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

