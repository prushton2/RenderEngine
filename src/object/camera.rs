use crate::ds;
use crate::object;
// use crate::material;

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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuUniform {
    pub pos:           [f32; 3],
    pub _pad0:         f32,      // pad vec3 to 16 bytes
    pub pixel00_loc:   [f32; 3],
    pub _pad1:         f32,
    pub pixel_delta_w: [f32; 3],
    pub _pad2:         f32,
    pub pixel_delta_h: [f32; 3],
    pub _pad3:         f32,
    pub width:         u32,
    pub height:        u32,
    pub sphere_count:  u32,
    pub _pad4:         u32,      // pad to multiple of 16
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

    pub fn to_gpu(&self) -> GpuUniform {
        GpuUniform {
            pos:           [self.pos.x as f32, self.pos.y as f32, self.pos.z as f32],
            _pad0: 0.0,
            pixel00_loc:   [self.pixel00_loc.x as f32, self.pixel00_loc.y as f32, self.pixel00_loc.z as f32],
            _pad1: 0.0,
            pixel_delta_w: [self.pixel_delta_w.x as f32, self.pixel_delta_w.y as f32, self.pixel_delta_w.z as f32],
            _pad2: 0.0,
            pixel_delta_h: [self.pixel_delta_h.x as f32, self.pixel_delta_h.y as f32, self.pixel_delta_h.z as f32],
            _pad3: 0.0,
            width: self.window_dimensions.0 as u32,
            height: self.window_dimensions.1 as u32,
            sphere_count: 0,
            _pad4: 0,
        }
    }

    // pub fn get_pixel_color(&self, world: &Vec<Box<dyn object::Renderable + Send + Sync>>, x: f64, y: f64) -> u32 {
    //     let pixel_center = self.pixel00_loc + (x * self.pixel_delta_w) + (y * self.pixel_delta_h);
    //     let ray_direction = pixel_center - self.pos();
    //     let ray = ds::Ray::new(&self.pos(), &ray_direction);

    //     return self.ray_color(world, &ray, 6).to_u32();
    // }

    // pub fn ray_color(&self, world: &Vec<Box<dyn object::Renderable + Send + Sync>>, ray: &ds::Ray, depth: u32) -> ds::Color {
    //     let mut lowest_distance: Option<f64> = None;
    //     let mut closest_object: Option<&(dyn object::Renderable + Send + Sync)> = None;

    //     let mut g_surface_pos: ds::Vector3 = ds::Vector3::zero();
    //     let mut g_t: f64 = 0.0;

        
    //     if depth <= 0 {
    //         return ds::Color::from_u32(0x00BADBED);
    //     }

    //     for renderable in world {
    //         let intersects = Some(8.0); //renderable.intersects(&ray);
    //         if intersects.is_none() || intersects.unwrap() < 0.0 {
    //             continue;
    //         }

    //         let t = intersects.unwrap();
    //         let surface_pos = ray.at(t);
    //         let len_sq = (surface_pos - ray.origin).length_sq();

    //         if lowest_distance.is_none() || len_sq < lowest_distance.unwrap() {
    //             lowest_distance = Some(len_sq);
    //             closest_object = Some(renderable.as_ref());
    //             g_surface_pos = surface_pos;
    //             g_t = t;
    //         }
    //     }

    //     return match closest_object {
    //         None => ds::Color::from_u32(0x00BADBED),
    //         Some(obj) => obj.get_material().ray_color(&self, obj, world, ray, g_t, &g_surface_pos, depth-1)
    //     };
    // }

    // pub fn set_pos(&mut self, pos: ds::Vector3) {
    //     self.pos = pos;
    //     let delta_dir = self.dir - self.pos;
    //     self.dir = pos;
    //     self.dirty = true;
    // }

    pub fn move_camera(&mut self, delta: ds::Vector3) {
        self.pos = self.pos + delta;
        self.dir = self.dir + delta;
        self.dirty = true;
    }

    // pub fn set_dir_absolute(&mut self, pos: ds::Vector3) {
    //     self.dir = pos;
    //     self.dirty = true;
    // }
    
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

    pub fn pos(&self) -> ds::Vector3 {
        return self.pos;
    }
}

unsafe impl Sync for Camera {}