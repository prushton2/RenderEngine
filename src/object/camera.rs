use crate::position;

pub struct Camera {
    // inputs
    pos: position::Vector3,
    dir: position::Vector3,
    up:  position::Vector3,

    window_dimensions: (f64, f64),
    focal_length: f64,
    vfov: f64,
    
    // outputs
    pixel_delta_w: position::Vector3,
    pixel_delta_h: position::Vector3,
    pixel00_loc: position::Vector3,
}

impl Camera {
    pub fn new(pos: position::Vector3, focal_length: f64, window_dimensions: (f64, f64), vfov: f64) -> Self{

        let angle: f64 = 1.0 * position::math::PI;

        let lookat = position::Vector3::new(angle.cos() * focal_length, 0.0, angle.sin() * focal_length);
        let up = position::Vector3::new(0.0, 1.0, 0.0);

        let mut this = Self {
            pos: pos,
            dir: lookat,
            up: up,            
            window_dimensions: window_dimensions,
            focal_length: 0.0,
            vfov: vfov,

            pixel_delta_h: position::Vector3::zero(),
            pixel_delta_w: position::Vector3::zero(),
            pixel00_loc: position::Vector3::zero(),
        };

        Self::update_outputs(&mut this);

        this
    }

    fn update_outputs(&mut self) {
        self.focal_length = (self.dir - self.pos).length();

        // man idk what this does
        let theta = position::math::degrees_to_radians(self.vfov);
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
        let viewport_upper_left_corner = self.pos + position::Vector3::new(0.0, 0.0, self.focal_length) * w - viewport_w/2.0 - viewport_h/2.0;

        // where is the upper left pixel
        self.pixel00_loc = viewport_upper_left_corner + (0.5 * (self.pixel_delta_w + self.pixel_delta_h));

    }

    pub fn move_camera(&mut self, delta: position::Vector3) {
        self.pos = self.pos + delta;
        self.dir = self.dir + delta;
        self.update_outputs();
    }

    pub fn turn_camera(&mut self, delta: position::Vector3) {

        // what do i even do lmao
        // let relative_dir = (self.dir - self.pos) / self.focal_length;
        // let angle = relative_dir.x.acos() * if relative_dir.z.asin() < 0.0 { -1.0 } else { 1.0 };
        // let angle = angle + delta.z;

        // let relative_dir = position::Vector3::new(angle.cos(), 0.0, angle.sin()) * self.focal_length;
        // self.dir = relative_dir + self.pos;

        println!("Looking at {:?}", self.dir);
        // println!("Angle {}", angle);

        // self.dir = self.pos + delta_dir;
        self.update_outputs();
    }

    pub fn pixel_delta_w(&self) -> position::Vector3 {
        return self.pixel_delta_w;
    }
    pub fn pixel_delta_h(&self) -> position::Vector3 {
        return self.pixel_delta_h;
    }
    pub fn pixel00_loc(&self) -> position::Vector3 {
        return self.pixel00_loc;
    }

    pub fn pos(&self) -> position::Vector3 {
        return self.pos;
    }
    // pub fn dir(&self) -> position::Ray {
    //     return self.dir;
    // }

}