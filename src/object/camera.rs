use crate::position;

pub struct Camera {
    // inputs
    pos: position::Vector3,
    dir: position::Ray,

    focal_length: f64,
    window_dimensions: (f64, f64),
    viewport_height: f64,

    // outputs
    pixel_delta_w: position::Vector3,
    pixel_delta_h: position::Vector3,
    pixel00_loc: position::Vector3
}

impl Camera {
    pub fn new(pos: position::Vector3, dir: position::Ray, focal_length: f64, window_dimensions: (f64, f64), viewport_height: f64) -> Self{

        let viewport_width = viewport_height * (window_dimensions.0 / window_dimensions.1);
        
        // just the width and height but as a vector 3 for easy math
        let viewport_w = position::Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_h = position::Vector3::new(0.0, -viewport_height, 0.0);

        // distance between pixels in terms of the raycast (how far to move the raycast between each pixel)
        let pixel_delta_w: position::Vector3 = viewport_w / window_dimensions.0;
        let pixel_delta_h: position::Vector3 = viewport_h / window_dimensions.1;

        let viewport_upper_left_corner = pos + position::Vector3::new(0.0, 0.0, focal_length) - viewport_w/2.0 - viewport_h/2.0;
        
        Self {
            pos: pos,
            dir: dir,

            focal_length: focal_length,
            window_dimensions: window_dimensions,
            viewport_height: viewport_height,
            
            pixel_delta_w: pixel_delta_w,
            pixel_delta_h: pixel_delta_h,

            pixel00_loc: viewport_upper_left_corner + (0.5 * (pixel_delta_w + pixel_delta_h))
        }
    }

    fn update_outputs(&mut self) {
        let viewport_width = self.viewport_height * (self.window_dimensions.0 / self.window_dimensions.1);
        
        // just the width and height but as a vector 3 for easy math
        let viewport_w = position::Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_h = position::Vector3::new(0.0, -self.viewport_height, 0.0);

        // distance between pixels in terms of the raycast (how far to move the raycast between each pixel)
        self.pixel_delta_w = viewport_w / self.window_dimensions.0;
        self.pixel_delta_h = viewport_h / self.window_dimensions.1;

        let viewport_upper_left_corner = self.pos + position::Vector3::new(0.0, 0.0, self.focal_length) - viewport_w/2.0 - viewport_h/2.0;

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
    pub fn dir(&self) -> position::Ray {
        return self.dir;
    }

}