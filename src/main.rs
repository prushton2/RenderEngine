use minifb::{Key, Window, WindowOptions};

mod position;
mod object;

const WIDTH: usize = 1280; //Resolution
const HEIGHT: usize = 720;

fn get_pixel_color(camera: &object::Camera, x: f64, y: f64) -> u32 {
    // let x = (i % WIDTH) as f64;
    // let y = (i / WIDTH) as f64;

    let pixel_center = camera.pixel00_loc + (x * camera.pixel_delta_w) + (y * camera.pixel_delta_h);
    let ray_direction = pixel_center - camera.pos;
    let ray = position::Ray::new(&camera.pos, &ray_direction);

    ((ray.direction.y+1.0) * 128.0) as u32
}

fn main() {
    let camera = object::Camera::new(
        position::Vector3::new(0.0, 0.0, 0.0),
        position::Ray::new(&position::Vector3::new(0.0, 0.0, 0.0), &position::Vector3::new(0.0, 0.0, 0.0)),
        1.0,
        (WIDTH as f64, HEIGHT as f64),
        2.0
    );

    minifbwindow(&camera);
}

fn minifbwindow(camera: &object::Camera) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Pixel Demo", 
        WIDTH, HEIGHT, 
        WindowOptions {
            borderless: false,  // true = no decorations at all
            title: true,        // show title bar
            resize: true,
        ..WindowOptions::default()
        }
    ).unwrap();


    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw pixels: format is 0x00RRGGBB
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[y * WIDTH + x] = get_pixel_color(&camera, x as f64, y as f64);
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}