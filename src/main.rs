use std::collections::HashMap;
use std::num::NonZeroU32;
use std::thread;
use std::sync::{Arc, RwLock};

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent, DeviceEvent, DeviceId};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId, CursorGrabMode};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::object::Renderable;

mod material;
mod object;
mod ds;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const THREAD_COUNT: usize = 32;
const SENSITIVITY: f64 = 0.001;
const SPEED: f64 = 1.5;

struct App {
    // window
    window: Option<Arc<Window>>,

    // wgpu (set in resumed, always Some after that)
    device:           Option<wgpu::Device>,
    queue:            Option<wgpu::Queue>,
    wgpu_surface:     Option<wgpu::Surface<'static>>,
    surface_config:   Option<wgpu::SurfaceConfiguration>,
    compute_pipeline: Option<wgpu::ComputePipeline>,
    render_pipeline:  Option<wgpu::RenderPipeline>,
    bind_group:       Option<wgpu::BindGroup>,
    camera_buf:       Option<wgpu::Buffer>,
    spheres_buf:      Option<wgpu::Buffer>,
    // quads_buf:        Option<wgpu::Buffer>,
    output_buf:       Option<wgpu::Buffer>,

    // scene
    player:  Arc<RwLock<object::Player>>,
    objects: Arc<Vec<Box<dyn object::Renderable + Send + Sync>>>,

    // input
    keyboard:    HashMap<KeyCode, bool>,
    mouse_delta: (f64, f64),

    // statistics
    last_frame:                 std::time::Instant,
    deltatime:                  f64,

    fps_over_last_second:       Vec<f64>,
    deltatime_over_last_second: Vec<f64>,
    average_fps:                u32,
    average_deltatime:          u32,
    statistics_timer:           std::time::Instant,
}

struct GpuState<'a> { // makes my life infinitely easier
    device:           &'a wgpu::Device,
    queue:            &'a wgpu::Queue,
    surface:          &'a wgpu::Surface<'static>,
    surface_config:   &'a wgpu::SurfaceConfiguration,
    compute_pipeline: &'a wgpu::ComputePipeline,
    render_pipeline:  &'a wgpu::RenderPipeline,
    bind_group:       &'a wgpu::BindGroup,
    camera_buf:       &'a wgpu::Buffer,
    spheres_buf:      &'a wgpu::Buffer,
    output_buf:       &'a wgpu::Buffer,
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
                player_ref.move_player(&(dir * SPEED * self.deltatime));
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

    fn get_gpu_state(&self) -> Option<GpuState> {
        Some(
            GpuState {
                device:           self.device.as_ref()?,
                queue:            self.queue.as_ref()?,
                surface:          self.wgpu_surface.as_ref()?,
                surface_config:   self.surface_config.as_ref()?,
                compute_pipeline: self.compute_pipeline.as_ref()?,
                render_pipeline:  self.render_pipeline.as_ref()?,
                bind_group:       self.bind_group.as_ref()?,
                camera_buf:       self.camera_buf.as_ref()?,
                spheres_buf:      self.spheres_buf.as_ref()?,
                // quads_buf:        self.quads_buf.as_ref()?,
                output_buf:       self.output_buf.as_ref()?,
            }
        )
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window:           None,
            device:           None,
            queue:            None,
            wgpu_surface:     None,
            surface_config:   None,
            compute_pipeline: None,
            render_pipeline:  None,
            bind_group:       None,
            camera_buf:       None,
            spheres_buf:      None,
            output_buf:       None,

            player:  Arc::new(RwLock::new(object::Player::new(object::Camera::zero()))),
            objects: Arc::new(vec![]),

            keyboard:    HashMap::new(),
            mouse_delta: (0.0, 0.0),

            last_frame:                 std::time::Instant::now(),
            deltatime:                  0.0,

            fps_over_last_second:       vec![],
            deltatime_over_last_second: vec![],
            average_fps:                0,
            average_deltatime:          0,
            statistics_timer:           std::time::Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(
                Window::default_attributes()
                    .with_title("Render Engine")
                    .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
            ).unwrap()
        );

        window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();
        window.set_cursor_visible(false);

        // wgpu init is async but resumed() isn't — use pollster to block
        let (device, queue, surface, surface_config,
            compute_pipeline, render_pipeline,
            bind_group, camera_buf, spheres_buf, output_buf
        ) = pollster::block_on(init_wgpu(window.clone(), WIDTH as u32, HEIGHT as u32));

        self.window           = Some(window);
        self.device           = Some(device);
        self.queue            = Some(queue);
        self.wgpu_surface     = Some(surface);
        self.surface_config   = Some(surface_config);
        self.compute_pipeline = Some(compute_pipeline);
        self.render_pipeline  = Some(render_pipeline);
        self.bind_group       = Some(bind_group);
        self.camera_buf       = Some(camera_buf);
        self.spheres_buf      = Some(spheres_buf);
        self.output_buf       = Some(output_buf);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.mouse_delta = (self.mouse_delta.0 + (dx as f64)*SENSITIVITY, self.mouse_delta.1 + (dy as f64)*SENSITIVITY);
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

                let gpu_state = match self.get_gpu_state() {
                    Some(t) => t,
                    None => return
                };

                let width: usize = size.width as usize;
                let height: usize = size.height as usize;
                
                self.player.write().unwrap().get_camera_mut().set_window_size(size.width.into(), size.height.into());
                self.handle_movement();
                
                // let surface = self.surface.as_mut().unwrap();
                // surface.resize(width_nzu32, height_nzu32).expect("Failed to resize surface");
                
                // let mut buf = surface.buffer_mut().expect("Failed to get buffer");
                
                // let mut threads = vec![];

                // for i in 0..THREAD_COUNT {
                //     let player_ref = Arc::clone(&self.player);
                //     let objects_ref = Arc::clone(&self.objects);

                //     threads.push(thread::spawn(move || {
                //         let strip_start = (height / THREAD_COUNT) * i;
                //         let strip_end = if i == THREAD_COUNT - 1 {
                //             height  // last thread takes any leftover rows
                //         } else {
                //             (height / THREAD_COUNT) * (i + 1)
                //         };
                //         let strip_height = strip_end - strip_start;

                //         let mut pixels: Vec<u32> = vec![0; width * strip_height];
                //         let player_read = player_ref.read().unwrap();

                //         for x in 0..width {
                //             for y in strip_start..strip_end {
                //                 pixels[(y - strip_start) * width + x] = player_read.get_camera().get_pixel_color(objects_ref.as_ref(), x as f64, y as f64);
                //             }
                //         }

                //         pixels
                //     }));
                // }

                // let mut pixels: Vec<u32> = Vec::with_capacity(width*height);

                // for thread in threads.drain(0..threads.len()) {
                //     pixels.extend(thread.join().unwrap());
                // }

                // buf.copy_from_slice(&pixels);

                // buf.present().expect("Failed to present buffer");

                let player = self.player.read().unwrap();

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
                println!(" FPS: {}\n\n Time between frames: {}ms\n\n Camera position: {:?}\n Player Rotation: {:?}", self.average_fps, self.average_deltatime, player.get_camera().pos(), player.get_rotation());
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

    let objects: Vec<Box<dyn Renderable + Send + Sync>> = vec![
        Box::new(object::Sphere::new(&ds::Vector3::new( 2.0,  0.0, 7.0), 0.5, Box::new(material::Molly::new(0.5, 1.0)))),

        Box::new(object::Sphere::new(&ds::Vector3::new(-1.0,  0.0, 7.0), 0.1, Box::new(material::Debug::new()))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-2.0, -0.4, 5.0), 0.1, Box::new(material::Debug::new()))),
        Box::new(object::Sphere::new(&ds::Vector3::new( 0.0,  0.0, 5.0), 0.5, Box::new(material::Debug::new()))),

        // Box::new(object::Quad::new(&ds::Vector3::new(-1.0, -1.0, 6.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Translucent::new(0x00333333)))),
        // Box::new(object::Quad::new(&ds::Vector3::new(-1.0, -1.0, 7.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Debug::new()))),

        // Box::new(object::Quad::new(&ds::Vector3::new(-2.0, -0.5, 2.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Translucent::new(0x00880000)))),
        // Box::new(object::Quad::new(&ds::Vector3::new(-3.0, -0.5, 2.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Translucent::new(0x00008800)))),
        // Box::new(object::Quad::new(&ds::Vector3::new(-4.0, -0.5, 2.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Translucent::new(0x00000088)))),

        // Box::new(object::Quad::new(&ds::Vector3::new(-5.0, -0.5, 3.0), &ds::Vector3::new(0.0, 0.0, 1.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Mirror::new(0x00FFFFFF)))),
        // Box::new(object::Quad::new(&ds::Vector3::new(-7.0, -0.5, 3.0), &ds::Vector3::new(0.0, 0.0, 1.0), &ds::Vector3::new(0.0, 1.0, 0.0), Box::new(material::Mirror::new(0x00000000)))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-6.0, 0.0, 3.5), 0.1, Box::new(material::StaticColor::new(0x000000FF)))),


        Box::new(object::Sphere::new(&ds::Vector3::new(-3.25, -0.8, 6.0), 0.5, Box::new(material::StaticColor::new(0x00FF0000)))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.75, -0.8, 6.0), 0.5, Box::new(material::StaticColor::new(0x000000FF)))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   0.0, 6.0), 0.5, Box::new(material::Absorb::new(0x00FFFFFF)))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   1.0, 6.0), 0.5, Box::new(material::StaticColor::new(0x0000FF00)))),
        Box::new(object::Sphere::new(&ds::Vector3::new(-4.0,   2.0, 6.0), 0.5, Box::new(material::Translucent::new(0x00333333))))
    ];

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    app.consume_player(player);
    app.consume_objects(objects);

    event_loop.run_app(&mut app).expect("Event loop failed");
}



// vibecoded but man thats a lot
async fn init_wgpu(window: Arc<Window>, width: u32, height: u32) -> (
    wgpu::Device,
    wgpu::Queue,
    wgpu::Surface<'static>,
    wgpu::SurfaceConfiguration,
    wgpu::ComputePipeline,
    wgpu::RenderPipeline,
    wgpu::BindGroup,
    wgpu::Buffer, // camera
    wgpu::Buffer, // spheres
    wgpu::Buffer, // output
) {
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(window).unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::default(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        trace: wgpu::Trace::Off
    }).await.unwrap();

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &surface_config);

    // --- buffers ---

    let camera_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("camera"),
        size: std::mem::size_of::<object::camera::GpuCamera>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let spheres_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("spheres"),
        size: (std::mem::size_of::<object::sphere::GpuSphere>() * 512) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // output buffer — one u32 per pixel
    let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output"),
        size: (width * height * 4) as u64, // 4 bytes per pixel
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    // --- pipelines ---

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("raytracer"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./../src/shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            // camera
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // spheres
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // output
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: camera_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: spheres_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: output_buf.as_entire_binding() },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(&bind_group_layout)],
        // push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    // minimal render pipeline — just blits the output buffer to screen
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("blit"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        // multiview: None,
        cache: None,
    });

    (device, queue, surface, surface_config, compute_pipeline,
     render_pipeline, bind_group, camera_buf, spheres_buf, output_buf)
}