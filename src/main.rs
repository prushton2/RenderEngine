#[allow(non_snake_case)]

mod position;
mod object;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


const WIDTH: u32 = 480; //Resolution
const HEIGHT: u32 = 320;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct Screen {
    bg_color: position::Vector3::vector3,
}

fn main() {
    // let mut res = [[[3u8; 3]; WIDTH as usize]; HEIGHT as usize];
    // res[0][0][0] = 128u8;
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

    screen.bg_color = position::Vector3::new(255.0, 255.0, 255.0);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            screen.draw(pixels.get_frame());
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

            if(input.key_pressed(VirtualKeyCode::Space)) {
                screen.bg_color = position::Vector3::new(0.0, 0.0, 1.0).add(&screen.bg_color);
                dbg!(&screen.bg_color.z);
            }

            if(input.key_pressed(VirtualKeyCode::LShift)) {
                screen.bg_color = position::Vector3::new(0.0, 0.0, -1.0).add(&screen.bg_color);
                dbg!(&screen.bg_color.z);
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

    fn paint(&mut self, pos: position::Vector3::vector3, color: position::Vector3::vector3) {
        
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        let mut prevHighest = 0;
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = self.bg_color.x.round() as u8;
            pixel[1] = self.bg_color.y.round() as u8;
            pixel[2] = self.bg_color.z.round() as u8;
            
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;


        }
    }
}