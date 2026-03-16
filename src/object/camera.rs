use crate::ds;
use crate::object;

use crate::ds::Color;

pub struct Camera {
    // inputs
    pos: ds::Vector3,
    dir: ds::Vector3,
    up:  ds::Vector3,
    dirty: bool,

    window_dimensions: (f64, f64),
    focal_length: f64,
    vfov: f64,
    
    // outputs
    pixel_delta_w: ds::Vector3,
    pixel_delta_h: ds::Vector3,
    pixel00_loc: ds::Vector3,
}

impl Camera {
    pub fn zero() -> Self { // irreedeemable
        Self {
            pos: ds::Vector3::zero(),
            dir: ds::Vector3::zero(),
            up: ds::Vector3::zero(),
            dirty: true,

            window_dimensions: (0.0, 0.0),
            focal_length: 0.0,
            vfov: 0.0,

            pixel_delta_h: ds::Vector3::zero(),
            pixel_delta_w: ds::Vector3::zero(),
            pixel00_loc: ds::Vector3::zero(),
        }

    }

    pub fn new(pos: ds::Vector3, focal_length: f64, window_dimensions: (f64, f64), vfov: f64) -> Self{

        // let angle: f64 = 0.5 * ds::math::PI;
        // let lookat = ds::Vector3::new(angle.cos() * focal_length, 0.0, angle.sin() * focal_length);
        let lookat = ds::Vector3::new(0.0, 0.0, focal_length);
        let up = ds::Vector3::new(0.0, 1.0, 0.0);

        let mut this = Self {
            pos: pos,
            dir: lookat,
            up: up,
            dirty: true,

            window_dimensions: window_dimensions,
            focal_length: 0.0,
            vfov: vfov,

            pixel_delta_h: ds::Vector3::zero(),
            pixel_delta_w: ds::Vector3::zero(),
            pixel00_loc: ds::Vector3::zero(),
        };

        Self::update_outputs(&mut this);

        this
    }

    pub fn update_outputs(&mut self) {
        if !self.dirty {
            return
        }
        self.dirty = false;

        self.focal_length = (self.dir - self.pos).length();

        // man idk what this does
        let theta = ds::math::degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();

        // viewport math (i understand all of this aside from h)
        let viewport_height = 2.0 * h * self.focal_length;
        let viewport_width = viewport_height * (self.window_dimensions.0/self.window_dimensions.1);

        // brother what
        let w = (self.dir - self.pos).unit_vector();
        let u = self.up.cross(&w).unit_vector();
        let v = w.cross(&u);

        // just the width and height but as a vector 3 for easy math
        let viewport_w = u * viewport_width;     // across horizontal edge
        let viewport_h = v * -viewport_height;   // down vertical edge

        // distance between pixels in terms of the raycast (how far to move the raycast between each pixel)
        self.pixel_delta_w = viewport_w / self.window_dimensions.0;
        self.pixel_delta_h = viewport_h / self.window_dimensions.1;

        // where is the upper left corner of the viewport
        let viewport_upper_left_corner = self.pos + w * self.focal_length - viewport_w/2.0 - viewport_h/2.0;

        // where is the upper left pixel
        self.pixel00_loc = viewport_upper_left_corner + (0.5 * (self.pixel_delta_w + self.pixel_delta_h));
    }

    pub fn get_pixel_color(&self, world: &Vec<Box<dyn object::Renderable + Send + Sync>>, x: f64, y: f64) -> u32 {
        let pixel_center = self.pixel00_loc + (x * self.pixel_delta_w) + (y * self.pixel_delta_h);
        let ray_direction = pixel_center - self.pos();
        let ray = ds::Ray::new(&self.pos(), &ray_direction);

        return self.ray_color(world, &ray, 5).to_u32();
    }

    pub fn ray_color(&self, world: &Vec<Box<dyn object::Renderable + Send + Sync>>, ray: &ds::Ray, depth: u32) -> Color {
        let mut lowest_distance: Option<f64> = None;
        let mut color = Color::from_u32(0x00BADBED);

        if depth == 0 {
            return color
        }

        for renderable in world {
            let intersects = renderable.intersects(&ray);
            
            // we dont intersect or its behind the camera
            if intersects.is_none() || intersects.unwrap() < 0.0 {
                continue
            }

            let t = intersects.unwrap();
            let surface_pos = ray.at(t);
            let len_sq = (surface_pos - ray.origin).length_sq();

            if lowest_distance == None || len_sq < lowest_distance.unwrap() {
                lowest_distance = Some(len_sq);
                color = match renderable.color(&surface_pos) {
                    object::renderable::ColorType::Rgb(c) => {
                        c
                    },
                    object::renderable::ColorType::Absorb(c) => {
                        let surface_normal = renderable.hit_record(ray, t).outward_surface_normal;
                        self.ray_color(world, &ds::Ray::new(&surface_pos, &surface_normal), depth-1)/2 + c/2
                    },
                    object::renderable::ColorType::Translucent(c) => {
                        self.ray_color(world, &ds::Ray::new(&ray.at(t+0.0000001), &ray.direction), depth-1)/2 + c/2
                    },
                    object::renderable::ColorType::Debug_shade => {
                        let n = (surface_pos - renderable.center()).unit_vector();
                        ds::Color::from_u32(((n.x*255.0) as u32) << 16 | ((n.y*255.0) as u32) << 8 | ((n.z*-255.0) as u32))
                    }
                };
            }
        }
        return color;
    }

    pub fn set_pos(&mut self, pos: ds::Vector3) {
        self.pos = pos;
        let delta_dir = self.dir - self.pos;
        self.dir = pos;
        self.dirty = true;
    }

    pub fn move_camera(&mut self, delta: ds::Vector3) {
        self.pos = self.pos + delta;
        self.dir = self.dir + delta;
        self.dirty = true;
    }

    pub fn set_dir_absolute(&mut self, pos: ds::Vector3) {
        self.dir = pos;
        self.dirty = true;
    }
    
    pub fn set_dir_relative(&mut self, dir: ds::Vector3) {
        self.dir = self.pos + (dir * self.focal_length);
        self.dirty = true;
    }

    pub fn set_window_size(&mut self, width: f64, height: f64) {
        if (width, height) != self.window_dimensions {
            self.window_dimensions = (width as f64, height as f64);
            self.dirty = true;
            self.update_outputs();
        }
    }

    pub fn pixel_delta_w(&self) -> ds::Vector3 {
        return self.pixel_delta_w;
    }
    pub fn pixel_delta_h(&self) -> ds::Vector3 {
        return self.pixel_delta_h;
    }
    pub fn pixel00_loc(&self) -> ds::Vector3 {
        return self.pixel00_loc;
    }

    pub fn pos(&self) -> ds::Vector3 {
        return self.pos;
    }
    pub fn dir_absolute(&self) -> ds::Vector3 {
        return self.dir;
    }

}

unsafe impl Sync for Camera {}