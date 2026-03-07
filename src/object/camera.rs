use crate::position;

pub struct Camera {
    pub pos: position::Vector3,
    pub dir: position::Ray,

    pub pixel_delta_w: position::Vector3,
    pub pixel_delta_h: position::Vector3,
    pub pixel00_loc: position::Vector3
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

        let viewport_upper_left_corner = pos - position::Vector3::new(0.0, 0.0, focal_length) - viewport_w/2.0 - viewport_h/2.0;

        Self {
            pos: pos,
            dir: dir,
            
            pixel_delta_w: pixel_delta_w,
            pixel_delta_h: pixel_delta_h,

            pixel00_loc: viewport_upper_left_corner + (0.5 * (pixel_delta_w + pixel_delta_h))
        }
    }
}