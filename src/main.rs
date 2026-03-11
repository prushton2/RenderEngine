use minifb::{Key, Window, WindowOptions};
use crate::object::renderable::Renderable;

mod position;
mod object;

const WIDTH: usize = 1280; //Resolution
const HEIGHT: usize = 720;

fn get_pixel_color(camera: &object::Camera, objects: &Vec<Box<dyn object::Renderable>>, x: f64, y: f64) -> u32 {
    let pixel_center = camera.pixel00_loc() + (x * camera.pixel_delta_w()) + (y * camera.pixel_delta_h());
    let ray_direction = pixel_center - camera.pos();
    let ray = position::Ray::new(&camera.pos(), &ray_direction);

    let mut lowest_distance: Option<f64> = None;
    let mut color = 0x00000088;

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
    let mut camera = object::Camera::new(
        position::Vector3::new(0.0, 0.0, 0.0),
        // position::Ray::new(&position::Vector3::new(0.0, 0.0, 0.0), &position::Vector3::new(0.0, 0.0, 0.0)),
        3.0,
        (WIDTH as f64, HEIGHT as f64),
        60.0
    );

    let objects: Vec<Box<dyn object::Renderable>> = vec![
        Box::new(object::Sphere::new(&position::Vector3::new(0.0, 0.0, 7.0), 0.1)),
        Box::new(object::Sphere::new(&position::Vector3::new(0.0, 0.0, 5.0), 0.5)),
        Box::new(object::Sphere::new(&position::Vector3::new(-2.0, -0.4, 5.0), 0.1))
        // Box::new(object::Sphere::new(&position::Vector3::new(-0.75, 0.2, 5.0), 0.5)),
        // Box::new(object::Sphere::new(&position::Vector3::new(0.75, 0.2, 5.0), 0.5)),
        // Box::new(object::Sphere::new(&position::Vector3::new(0.0, 1.0, 5.0), 0.5)),
        // Box::new(object::Sphere::new(&position::Vector3::new(0.0, 2.0, 5.0), 0.5)),
        // Box::new(object::Sphere::new(&position::Vector3::new(0.0, 3.0, 5.0), 0.5))
    ];

    // let sphere = object::Sphere::new(&position::Vector3::new(0.0, 0.0, 5.0), 0.5);

    minifbwindow(&mut camera, &objects);
}

fn minifbwindow(camera: &mut object::Camera, objects: &Vec<Box<dyn object::Renderable>>) {
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
        let start = std::time::Instant::now();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[y * WIDTH + x] = get_pixel_color(&camera, &objects, x as f64, y as f64);
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        let elapsed = start.elapsed();
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\n Time between frames: {}ms\n\n Camera position: {:?}", elapsed.as_millis(), camera.pos());
        
        if elapsed.as_millis() < (1000.0/60.0) as u128 {
            std::thread::sleep(std::time::Duration::from_millis((1000.0/60.0) as u64 - elapsed.as_millis() as u64));
        }

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
        if window.is_key_down(Key::Left) {
            camera.turn_camera(position::Vector3::new(0.0, 0.0, 0.05));
        }
        if window.is_key_down(Key::Right) {
            camera.turn_camera(position::Vector3::new(0.0, 0.0, -0.05));
        }
    }
}