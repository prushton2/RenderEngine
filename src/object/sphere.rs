use crate::position;
use crate::object::Renderable;

pub struct Sphere {
    center: position::Vector3,
    radius: f64
}

impl Sphere {
    pub fn new(center: &position::Vector3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius: radius
        }
    }
}

impl Renderable for Sphere {
    fn intersects(&self, ray: &position::Ray) -> Option<f64> {
        let a = ray.direction.length_sq();
        let h = ray.direction.dot(&(self.center - ray.origin));
        let c = (self.center - ray.origin).length_sq() - self.radius * self.radius;

        let discriminant = h*h-a*c;

        if discriminant < 0.0 {
            return None;
        } else {
            return Some((h - discriminant.sqrt() ) / a);
        }
    }

    fn color(&self, surface_pos: &position::Vector3) -> u32 {
        let n = (surface_pos - self.center).unit_vector();
        ((n.x*255.0) as u32) << 16 | ((n.y*255.0) as u32) << 8 | ((n.z*-255.0) as u32)
        // ((n.y*128.0) as u32) << 16 | ((n.y*37.5) as u32) << 8 | ((n.y*40.0) as u32) +
        // 0x00802528
        // ((n.y*-128.0) as u32) << 16 | ((n.y*-37.5) as u32) << 8 | ((n.y*-40.0) as u32)
    }
}