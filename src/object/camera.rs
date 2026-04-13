use crate::ds;
// use crate::object;
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
    pub pos:            [f32; 3],
    pub width:           u32,
    pub pixel00_loc:    [f32; 3],
    pub height:          u32,
    pub pixel_delta_w:  [f32; 3],
    pub sphere_count:    u32,
    pub pixel_delta_h:  [f32; 3],
    pub quad_count:      u32,
    pub texture_count:   u32,
    pub _pad_end:       [u32; 47], // pad to 256 bytes
}

impl Camera {
    pub fn zero() -> Self { // Basically a zero to fill some needs for a "useless" implementation
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
            pixel00_loc:   [self.pixel00_loc.x as f32, self.pixel00_loc.y as f32, self.pixel00_loc.z as f32],
            pixel_delta_w: [self.pixel_delta_w.x as f32, self.pixel_delta_w.y as f32, self.pixel_delta_w.z as f32],
            pixel_delta_h: [self.pixel_delta_h.x as f32, self.pixel_delta_h.y as f32, self.pixel_delta_h.z as f32],
            width: self.window_dimensions.0 as u32,
            height: self.window_dimensions.1 as u32,
            sphere_count: 0,
            quad_count: 0,
            texture_count: 0,
            _pad_end: [0u32; 47],
        }
    }

    pub fn move_camera(&mut self, delta: ds::Vector3) {
        self.pos = self.pos + delta;
        self.dir = self.dir + delta;
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

    pub fn pos(&self) -> ds::Vector3 {
        return self.pos;
    }
    pub fn dir(&self) -> ds::Vector3 {
        return self.dir;
    }
}