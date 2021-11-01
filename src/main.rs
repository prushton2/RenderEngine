#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod position;
mod object;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1280; //Resolution
const HEIGHT: u32 = 720;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct Screen {
    bg_color: position::Vector3::vector3,
}

fn main() {
    run();
}


fn run() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Render Engine")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut screen = Screen::new();

    screen.set_bg_color(pixels.get_frame(), position::Vector3::new(255.0, 255.0, 255.0));

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // screen.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::R) {
                screen.set_bg_color(pixels.get_frame(), position::Vector3::new(255.0, 255.0, 255.0));
            }

            if input.key_pressed(VirtualKeyCode::F1) {
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(120.0, 100.0, 0.0), &position::Vector3::new(100.0, 200.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(120.0, 100.0, 0.0), &position::Vector3::new(220.0, 100.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(100.0, 200.0, 0.0), &position::Vector3::new(200.0, 200.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(220.0, 100.0, 0.0), &position::Vector3::new(200.0, 200.0, 0.0));

            }

            if input.key_pressed(VirtualKeyCode::F2) {
                let pos1 = position::Vector3::new(420.0, 110.0, 0.0);
                let pos2 = position::Vector3::new(450.0, 160.0, 0.0);
                screen.drawLine(pixels.get_frame(), &pos1, &pos2);
                screen.draw(pixels.get_frame(), &pos1, &position::Vector3::new(255.0, 0.0, 0.0));
                screen.draw(pixels.get_frame(), &pos2, &position::Vector3::new(255.0, 0.0, 0.0));
            }

            if input.key_pressed(VirtualKeyCode::F3) {
                screen.boundaryFill4(pixels.get_frame(), &position::Vector3::new(101.0, 101.0, 0.0));
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // let mouse_diff = input.mouse_diff();
            // if mouse_diff != (0.0, 0.0) {
            //     println!("The mouse diff is: {:?}", mouse_diff);
            //     println!("The mouse position is: {:?}", input.mouse());
            // }


            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

impl Screen {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            bg_color: position::Vector3::new(0.0, 0.0, 0.0),
        }
    }

    fn render(&mut self, pos: position::Vector3::vector3, color: position::Vector3::vector3) {
        
    }

    fn getPixelColor(&mut self, frame:&mut [u8], pos: &position::Vector3::vector3) -> position::Vector3::vector3 {
        let index: usize = (pos.y as u32 * WIDTH as u32 + pos.x as u32) as usize;
        position::Vector3::new(frame[index*4+0] as f64, frame[index*4+1] as f64, frame[index*4+2] as f64)
    }

    fn boundaryFill4(&mut self, frame: &mut [u8], start: &position::Vector3::vector3) {
        let mut stack = Vec::new();
        stack.push(start.clone());

        while stack.len() > 0 {
            let currentPos = stack[stack.len()-1].clone();
            let currentPixel = self.getPixelColor(frame, &currentPos);
            if !currentPixel.eq(&position::Vector3::new(0.0, 0.0, 0.0)) {
                self.draw(frame, &currentPos, &position::Vector3::new(0.0, 0.0, 0.0));
                stack.push(position::Vector3::new(currentPos.x + 1.0,  currentPos.y + 0.0,  currentPos.z));
                stack.push(position::Vector3::new(currentPos.x + 0.0,  currentPos.y + 1.0,  currentPos.z));
                stack.push(position::Vector3::new(currentPos.x + -1.0, currentPos.y + 0.0,  currentPos.z));
                stack.push(position::Vector3::new(currentPos.x + 0.0,  currentPos.y + -1.0, currentPos.z));
            } else {
                stack.pop();
            }
        }
    }

    fn drawLine(&mut self, frame: &mut [u8], pos1: &position::Vector3::vector3, pos2: &position::Vector3::vector3) { //pos1.x must be less than pos2.x
        let mut pos1 = pos1;
        let mut pos2 = pos2;
        if(pos1.x > pos2.x) {
            let pos3 = pos2;
            pos2 = pos1;
            pos1 = pos3;
        }

        let x_diff = pos2.x - pos1.x;
        let y_diff = pos2.y - pos1.y;

        let slope = y_diff/x_diff;

        if x_diff == 0.0 {// this needs to be improved
            for i in 0..y_diff as i64 {
                self.draw(frame, &position::Vector3::new(pos1.x, pos1.y + i as f64, 0.0), &position::Vector3::new(0.0, 0.0, 0.0));
            }
        }

        for i in 0..x_diff as i64 {
            self.draw(frame, &position::Vector3::new(pos1.x + i as f64, pos1.y + (i as f64*slope), 0.0), &position::Vector3::new(0.0, 0.0, 0.0));
        }

    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8], pos: &position::Vector3::vector3, color: &position::Vector3::vector3) {
        
        let index: usize = (pos.y as u32 * WIDTH as u32 + pos.x as u32) as usize;

        frame[index*4+0] = color.x.round() as u8;
        frame[index*4+1] = color.y.round() as u8;
        frame[index*4+2] = color.z.round() as u8;
        frame[index*4+3] = 0u8;
    }

    fn set_bg_color(&self, frame: &mut [u8], color: position::Vector3::vector3) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = color.x.round() as u8;
            pixel[1] = color.y.round() as u8;
            pixel[2] = color.z.round() as u8;
        }
    }
}