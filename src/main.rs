#[allow(non_snake_case)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use position;
use object;


const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
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

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame(), position::Vector3::new(0.0, 0.0, 0.0));
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

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

fn draw(frame: &mut [u8], spot: position::vector3) {



    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i16;
        let y = (i / WIDTH as usize) as i16;

        let inside_the_box = x >= self.box_x
            && x < self.box_x + BOX_SIZE
            && y >= self.box_y
            && y < self.box_y + BOX_SIZE;

        let rgba = if inside_the_box {
            [0x5e, 0x48, 0xe8, 0xff]
        } else {
            [0x48, 0xb2, 0xe8, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }
}



















// mod position;
// mod object;

// fn main() {

//     let cube = object::Cube::new(
//         &position::Vector3::new(0.0, 0.0, 0.0),
//         &position::Vector3::new(1.0, 1.0, 1.0),
//     );
// }