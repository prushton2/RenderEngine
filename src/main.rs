#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod position;
mod object;
mod math;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::cmp;

use bresenham::Bresenham;

const WIDTH: u32 = 600; //Resolution
const HEIGHT: u32 = 400;


/// Representation of the application state. In this example, a box will bounce around the screen.
struct Screen {
    bg_color: position::Vector3::vector3,
    triangles: Vec<object::Triangle::triangle>,
    camera: object::Camera::camera,
    scalar: position::Vector3::vector3,
    // emptyScreen: [u8; (WIDTH*HEIGHT) as usize],
}



fn main() {

    run(createObjects());
}

fn createObjects() -> Vec<object::Triangle::triangle> {
    let cube1 = object::Cube::new(&position::Vector3::new(-1.0, 1.0, 1.0), &position::Vector3::new(1.0, 3.0, 3.0));
    let mut returnVec = Vec::new();
    // returnVec.push(object::Triangle::new(
    //     position::Vector3::new(1.0, 1.0, 1.0), 
    //     position::Vector3::new(1.0, 2.0, 1.0), 
    //     position::Vector3::new(2.0, 1.0, 1.0)
    //     )
    // );
    for i in cube1.getTriangles() {
        returnVec.push(i.clone());
    }
    returnVec
}

fn run(triangles: Vec<object::Triangle::triangle>) -> Result<(), Error> {
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

    screen.triangles = triangles;

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

            if input.key_pressed(VirtualKeyCode::W) {
                screen.render(pixels.get_frame());
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
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(50.0, 50.0, 0.0), &position::Vector3::new(50.0, 250.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(50.0, 50.0, 0.0), &position::Vector3::new(250.0, 50.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(50.0, 250.0, 0.0), &position::Vector3::new(250.0, 250.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(250.0, 50.0, 0.0), &position::Vector3::new(250.0, 250.0, 0.0));

            }

            if input.key_pressed(VirtualKeyCode::F3) {
                screen.boundaryFill4(pixels.get_frame(), &position::Vector3::new(150.0, 150.0, 0.0));
            }

            if input.key_pressed(VirtualKeyCode::F4) {
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(0.0, 0.0, 0.0), &position::Vector3::new(WIDTH as f64 - 1.0, 0.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(WIDTH as f64 - 1.0, HEIGHT as f64 - 1.0, 0.0), &position::Vector3::new(WIDTH as f64 - 1.0, 0.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(WIDTH as f64 - 1.0, HEIGHT as f64 - 1.0, 0.0), &position::Vector3::new(0.0, HEIGHT as f64 - 1.0, 0.0));
                screen.drawLine(pixels.get_frame(), &position::Vector3::new(0.0, HEIGHT as f64 - 1.0, 0.0), &position::Vector3::new(0.0, 0.0, 0.0));
            
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
            triangles: Vec::new(),
            camera: object::Camera::new(
                    position::Vector3::new(0.0, 0.0, 0.0),
                    position::Rotation::new(position::Angle::new(0.0), position::Angle::new(0.0), position::Angle::new(0.0)),
                    position::Rotation::new(position::Angle::new(0.0), position::Angle::new(0.0), position::Angle::new(0.0))
                ),
            scalar: position::Vector3::new(1.0, 1.0, 1.0),
            // emptyScreen: [0u8; (WIDTH*HEIGHT) as usize],
        }
    }

    fn render(&mut self, frame: &mut [u8]) {
        // self.drawLine(frame, &position::Vector3::new(WIDTH as f64 / 2.0, 0.0, 0.0), &position::Vector3::new(WIDTH as f64 / 2.0, HEIGHT as f64, 0.0));
        for i in 0..self.triangles.len() {

            let i = &self.triangles[i];
            
            println!("----------------------------------");
            println!("{}, {}, {}", i.pos1.x, i.pos1.y, i.pos1.z);
            println!("{}, {}, {}", i.pos2.x, i.pos2.y, i.pos2.z);
            println!("{}, {}, {}", i.pos3.x, i.pos3.y, i.pos3.z);
            
            let angle1 = math::getAnglesToPoint(&self.camera, &i.pos1.clone());
            let angle2 = math::getAnglesToPoint(&self.camera, &i.pos2.clone());
            let angle3 = math::getAnglesToPoint(&self.camera, &i.pos3.clone());
           
            println!("-------------");
            println!("{}, {}, {}", angle1.x.angle, angle1.y.angle, angle1.z.angle);
            println!("{}, {}, {}", angle2.x.angle, angle2.y.angle, angle2.z.angle);
            println!("{}, {}, {}", angle3.x.angle, angle3.y.angle, angle3.z.angle);
            
            let pos1 = self.angleToPixel(angle1);
            let pos2 = self.angleToPixel(angle2);
            let pos3 = self.angleToPixel(angle3);

            println!("-------------");
            println!("{}, {}, {}", pos1.x, pos1.y, pos1.z);
            println!("{}, {}, {}", pos2.x, pos2.y, pos2.z);
            println!("{}, {}, {}", pos3.x, pos3.y, pos3.z);
            

            self.drawLine(frame, &pos1.mult(&self.scalar), &pos2.mult(&self.scalar));
            self.drawLine(frame, &pos2.mult(&self.scalar), &pos3.mult(&self.scalar));
            self.drawLine(frame, &pos3.mult(&self.scalar), &pos1.mult(&self.scalar));

            // let center = ((pos1.add(&pos2)).add(&pos3)).div(&position::Vector3::new(3.0, 3.0, 3.0));

            let mut center = pos1.add(&pos2);
            center = center.add(&pos3);
            center = center.div(&position::Vector3::new(3.0, 3.0, 3.0));
            // self.boundaryFill4(frame, &center);
            self.draw(frame, &center, &position::Vector3::new(255.0, 0.0, 0.0));
            
        }
    }

    fn angleToPixel(&mut self, angle: position::Rotation::rotation) -> position::Vector3::vector3 {
        const MID: u32 = WIDTH/2;

        let mut newPos = position::Vector3::new(0.0, 0.0, 0.0);


        if angle.x.angle > 180.0 {
            newPos.x = -1.0 * (360.0 - angle.x.angle) + MID as f64;
        } else {
            newPos.x = angle.x.angle as f64 + MID as f64;
        }
        newPos.y = HEIGHT as f64 - angle.y.angle;

        // newPos.x =  if newPos.x > WIDTH as f64 { newPos.x - WIDTH as f64 } else { newPos.x };

        newPos.x = newPos.x.ceil();
        newPos.y = newPos.y.ceil();
        newPos
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
        let mut start;
        let mut end;
        if pos2.x < pos1.x {
            start = pos2;
            end = pos1;
        } else {
            start = pos1;
            end = pos2;
        }
        
        for (x, y) in Bresenham::new((start.x as isize, start.y as isize), (end.x as isize, end.y as isize)) {
            self.draw(frame, &position::Vector3::new(x as f64, y as f64, 0.0), &position::Vector3::new(0.0, 0.0, 0.0));
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