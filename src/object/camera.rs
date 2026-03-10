use crate::position;

pub struct Camera {
    // inputs
    pos: position::Vector3,
    dir: position::Vector3,
    up:  position::Vector3,

    window_dimensions: (f64, f64),
    viewport_height: f64,
    focal_length: f64,
    
    // outputs
    pixel_delta_w: position::Vector3,
    pixel_delta_h: position::Vector3,
    pixel00_loc: position::Vector3,

    // stuff for rotation
    u: position::Vector3,
    v: position::Vector3,
    w: position::Vector3,
}

impl Camera {
    pub fn new(pos: position::Vector3, focal_length: f64, window_dimensions: (f64, f64), viewport_height: f64) -> Self{

        let lookat = position::Vector3::new(0.0, 0.0, 3.0);
        let up = position::Vector3::new(0.0, 1.0, 0.0);

        let mut this = Self {
            pos: pos,
            dir: lookat,
            up: up,

            window_dimensions: window_dimensions,
            viewport_height: viewport_height,
            focal_length: 0.0,

            pixel_delta_h: position::Vector3::zero(),
            pixel_delta_w: position::Vector3::zero(),
            pixel00_loc: position::Vector3::zero(),

            u: position::Vector3::zero(),
            v: position::Vector3::zero(),
            w: position::Vector3::zero()
        };

        Self::update_outputs(&mut this);

        this
    }

    fn update_outputs(&mut self) {
        self.focal_length = (self.pos - self.dir).length();

        // calculate the width given the aspect ration and height. This is basically fov.
        let viewport_width = self.viewport_height * (self.window_dimensions.0 / self.window_dimensions.1);
        
        // just the width and height but as a vector 3 for easy math
        let viewport_w = position::Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_h = position::Vector3::new(0.0, -self.viewport_height, 0.0);

        // distance between pixels in terms of the raycast (how far to move the raycast between each pixel)
        self.pixel_delta_w = viewport_w / self.window_dimensions.0;
        self.pixel_delta_h = viewport_h / self.window_dimensions.1;

        // where is the upper left corner of the viewport
        let viewport_upper_left_corner = self.pos + position::Vector3::new(0.0, 0.0, self.focal_length) - viewport_w/2.0 - viewport_h/2.0;

        // where is the upper left pixel
        self.pixel00_loc = viewport_upper_left_corner + (0.5 * (self.pixel_delta_w + self.pixel_delta_h));

    }

    pub fn move_camera(&mut self, delta: position::Vector3) {
        self.pos = self.pos + delta;
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