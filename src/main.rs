// #![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(non_camel_case_types)]

mod position;
mod object;
// mod math;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
// use std::cmp;

// use bresenham::Bresenham;

const WIDTH: u32 = 1280; //Resolution
const HEIGHT: u32 = 720;

fn ray_color(ray: &position::Ray) -> (u8, u8, u8) {
    (0, 0, ((ray.direction.y+1.0) * 128.0) as u8)
}

fn main() {
    let camera = object::Camera::new(
        position::Vector3::new(0.0, 0.0, 0.0),
        position::Ray::new(&position::Vector3::new(0.0, 0.0, 0.0), &position::Vector3::new(0.0, 0.0, 0.0)),
        1.0,
        (WIDTH as f64, HEIGHT as f64),
        2.0
    );

    // event loop
    let event_loop = EventLoop::new();

    // create window
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop).unwrap();

    // a surface texture is the bridge between the pixel buffer and the window
    let surface = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

    // get a mutable reference to the raw RGBA pixel buffer
    let frame = pixels.get_frame();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i as u32 % WIDTH) as f64;
        let y = (i as u32 / WIDTH) as f64;

        let pixel_center = camera.pixel00_loc + (x * camera.pixel_delta_w) + (y * camera.pixel_delta_h);
        let ray_direction = pixel_center - camera.pos;
        let ray = position::Ray::new(&camera.pos, &ray_direction);

        let color = ray_color(&ray);

        println!("{} {} {}", ray.direction.x, ray.direction.y, ray.direction.z);

        pixel[0] = color.0; // R
        pixel[1] = color.1; // G
        pixel[2] = color.2; // B
        pixel[3] = 0xff; // A
    }

    // put pixels into surface or frame idk for sure
    pixels.render().unwrap();

    // actual event loop, we move stuff into here since this is the lifetime of the window now
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // poll for events

        // switch on event
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::RedrawRequested(_) => {
                // draw your pixels here
                let frame = pixels.get_frame();
                // ... fill frame ...
                pixels.render().unwrap();
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}
