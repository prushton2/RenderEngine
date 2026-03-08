use minifb::{Key, Window, WindowOptions};
use crate::object::renderable::Renderable;

mod position;
mod object;

const WIDTH: usize = 1280; //Resolution
const HEIGHT: usize = 720;

fn get_pixel_color(camera: &object::Camera, sphere: &object::Sphere, x: f64, y: f64) -> u32 {
    let pixel_center = camera.pixel00_loc() + (x * camera.pixel_delta_w()) + (y * camera.pixel_delta_h());
    let ray_direction = (pixel_center - camera.pos()).normalize();
    let ray = position::Ray::new(&camera.pos(), &ray_direction);

    if sphere.intersects(&ray) {
        0x00FF0000
    } else {
        ((ray.direction.y+1.0) * 128.0) as u32
    }
}

fn main() {
    let mut camera = object::Camera::new(
        position::Vector3::new(0.0, 0.0, 0.0),
        position::Ray::new(&position::Vector3::new(0.0, 0.0, 0.0), &position::Vector3::new(0.0, 0.0, 0.0)),
        3.0,
        (WIDTH as f64, HEIGHT as f64),
        2.0
    );

    let sphere = object::Sphere::new(&position::Vector3::new(0.0, 0.0, 5.0), 0.5);

    minifbwindow(&mut camera, &sphere);
}

fn minifbwindow(camera: &mut object::Camera, sphere: &object::Sphere) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Render Engine",
        WIDTH, HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: true,
        ..WindowOptions::default()
        }
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // format is 0x00RRGGBB
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[y * WIDTH + x] = get_pixel_color(&camera, &sphere, x as f64, y as f64);
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(std::time::Duration::from_millis((1000.0/60.0) as u64));

        if window.is_key_down(Key::S) {
            camera.move_camera(position::Vector3::new(0.0, 0.0, -0.1));
        }
        if window.is_key_down(Key::W) {
            camera.move_camera(position::Vector3::new(0.0, 0.0, 0.1));
        }
        if window.is_key_down(Key::A) {
            camera.move_camera(position::Vector3::new(-0.1, 0.0, 0.0));
        }
        if window.is_key_down(Key::D) {
            camera.move_camera(position::Vector3::new(0.1, 0.0, 0.0));
        }
        if window.is_key_down(Key::Space) {
            camera.move_camera(position::Vector3::new(0.0, 0.1, 0.0));
        }
        if window.is_key_down(Key::LeftCtrl) {
            camera.move_camera(position::Vector3::new(0.0, -0.1, 0.0));
        }
    }
}