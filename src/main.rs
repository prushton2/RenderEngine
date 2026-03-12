use std::thread;
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

    let mut objects: Vec<Box<dyn object::Renderable + Send + Sync>> = vec![
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 0.0, 7.0), 0.1)),
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 0.0, 5.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(-2.0, -0.4, 5.0), 0.1)),

        Box::new(object::Quad::new(&ds::Vector3::new(-1.0, -1.0, 6.0), &ds::Vector3::new(1.0, 0.0, 0.0), &ds::Vector3::new(0.0, 1.0, 0.0))),

        Box::new(object::Sphere::new(&ds::Vector3::new(-0.75, -0.8, -5.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(0.75, -0.8, -5.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 0.0, -5.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 1.0, -5.0), 0.5)),
        Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 2.0, -5.0), 0.5))
    ];

    if false {
        for _i in 0..100 {
            objects.push(Box::new(object::Sphere::new(&ds::Vector3::new(0.0, 0.0, 5.0), 0.5)));
        }
    }

    minifbwindow(&mut player, &objects);
}

fn minifbwindow(player: &mut object::Player, objects: &Vec<Box<dyn object::Renderable + Send + Sync>>) {
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

        player.update_outputs();

        let mut pixel_buffer_0: [[u32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
        let mut pixel_buffer_1: [[u32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
        let mut pixel_buffer_2: [[u32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
        let mut pixel_buffer_3: [[u32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];

        let thread0 = thread::spawn(|| {render_thread(&player, objects, 0, 4, &mut pixel_buffer_0)});
        let thread1 = thread::spawn(|| {render_thread(&player, objects, 1, 4, &mut pixel_buffer_1)});
        let thread2 = thread::spawn(|| {render_thread(&player, objects, 2, 4, &mut pixel_buffer_2)});
        let thread3 = thread::spawn(|| {render_thread(&player, objects, 3, 4, &mut pixel_buffer_3)});

        thread0.join();
        thread1.join();
        thread2.join();
        thread3.join();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // buffer[y * WIDTH + x] = get_pixel_color(player.get_camera(), &objects, x as f64, y as f64);
                buffer[y * WIDTH + x] = match (y * WIDTH + x)%4 {
                    0 => pixel_buffer_0[x][y],
                    1 => pixel_buffer_1[x][y],
                    2 => pixel_buffer_2[x][y],
                    3 => pixel_buffer_3[x][y],
                    _ => {0}
                }
            }
        }

        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        let elapsed = start.elapsed();
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\n FPS: {}\n\n Time between frames: {}ms\n\n Camera position: {:?}\n\n Player Rotation: {:?}", 1000/elapsed.as_millis(), elapsed.as_millis(), player.get_camera().pos(), player.get_rotation());

        let deltatime: f64 = (elapsed.as_millis() as f64) / 1000.0;

        if window.is_key_down(Key::W) {
            player.move_player(&(ds::Vector3::new( 0.0,  0.0,  1.0) * deltatime));
        }
        if window.is_key_down(Key::S) {
            player.move_player(&(ds::Vector3::new( 0.0,  0.0, -1.0) * deltatime));
        }
        if window.is_key_down(Key::A) {
            player.move_player(&(ds::Vector3::new(-1.0,  0.0,  0.0) * deltatime));
        }
        if window.is_key_down(Key::D) {
            player.move_player(&(ds::Vector3::new( 1.0,  0.0,  0.0) * deltatime));
        }
        if window.is_key_down(Key::Space) {
            player.move_player(&(ds::Vector3::new( 0.0,  1.0,  0.0) * deltatime));
        }
        if window.is_key_down(Key::LeftCtrl) {
            player.move_player(&(ds::Vector3::new( 0.0, -1.0,  0.0) * deltatime));
        }

        if window.is_key_down(Key::Left) {
            player.change_rotation(ds::Vector3::new( 0.0, 0.0, -0.5) * deltatime);
        }
        if window.is_key_down(Key::Right) {
            player.change_rotation(ds::Vector3::new( 0.0, 0.0,  0.5) * deltatime);
        }

        if window.is_key_down(Key::Up) {
            player.change_rotation(ds::Vector3::new( 0.5, 0.0,  0.0) * deltatime);
        }
        if window.is_key_down(Key::Down) {
            player.change_rotation(ds::Vector3::new(-0.5, 0.0,  0.0) * deltatime);
        }
    }
}

fn render_thread(player: &object::Player, objects: &Vec<Box<dyn object::Renderable + Send + Sync>>, offset: usize, thread_count: usize, buffer: &mut [[u32; WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT/thread_count {
        for x in 0..WIDTH/thread_count {
            buffer[x][y] = get_pixel_color(player.get_camera(), &objects, (offset + x*thread_count) as f64, (y*thread_count) as f64);
        }
    }
}