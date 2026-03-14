use std::thread;
use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use minifb::{Key, Window, WindowOptions};
use crate::object::renderable::Renderable;

mod ds;
mod object;

const WIDTH: usize = 1280; //Resolution
const HEIGHT: usize = 720;

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

    let mut player = object::Player::new(
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

    let thread_count = 8;
    let player = Arc::new(RwLock::new(player));
    let objects = Arc::new(objects);
    let mut deltatime = 0.0;
    let mut ms_until_fps_update: i64 = 1000;
    let mut fps: u128 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = std::time::Instant::now();

        let mut threads = vec![];

        for i in 0..thread_count {
            let player_ref = Arc::clone(&player);
            let objects_ref = Arc::clone(&objects);

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

        window.update_with_buffer(&pixels, WIDTH, HEIGHT).unwrap();

        let mut player_mut = player.write().unwrap();

        if window.is_key_down(Key::W) {
            player_mut.move_player(&(ds::Vector3::new( 0.0,  0.0,  1.0) * deltatime));
        }
        if window.is_key_down(Key::S) {
            player_mut.move_player(&(ds::Vector3::new( 0.0,  0.0, -1.0) * deltatime));
        }
        if window.is_key_down(Key::A) {
            player_mut.move_player(&(ds::Vector3::new(-1.0,  0.0,  0.0) * deltatime));
        }
        if window.is_key_down(Key::D) {
            player_mut.move_player(&(ds::Vector3::new( 1.0,  0.0,  0.0) * deltatime));
        }
        if window.is_key_down(Key::Space) {
            player_mut.move_player(&(ds::Vector3::new( 0.0,  1.0,  0.0) * deltatime));
        }
        if window.is_key_down(Key::LeftCtrl) {
            player_mut.move_player(&(ds::Vector3::new( 0.0, -1.0,  0.0) * deltatime));
        }

        if window.is_key_down(Key::Left) {
            player_mut.change_rotation(ds::Vector3::new( 0.0, 0.0, -0.5) * deltatime);
        }
        if window.is_key_down(Key::Right) {
            player_mut.change_rotation(ds::Vector3::new( 0.0, 0.0,  0.5) * deltatime);
        }

        if window.is_key_down(Key::Up) {
            player_mut.change_rotation(ds::Vector3::new( 0.5, 0.0,  0.0) * deltatime);
        }
        if window.is_key_down(Key::Down) {
            player_mut.change_rotation(ds::Vector3::new(-0.5, 0.0,  0.0) * deltatime);
        }
        player_mut.update_outputs();

        let elapsed = start.elapsed();
        deltatime = (elapsed.as_millis() as f64) / 1000.0;
        ms_until_fps_update -= elapsed.as_millis() as i64;

        if ms_until_fps_update <= 0 {
            fps = 1000/elapsed.as_millis();
            ms_until_fps_update = 1000;
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("\n\n FPS: {}\n\n Time between frames: {}ms\n\n Camera position: {:?}\n\n Player Rotation: {:?}", fps, elapsed.as_millis(), player_mut.get_camera().pos(), player_mut.get_rotation());
    }
}