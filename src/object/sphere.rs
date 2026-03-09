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
        
        // return discriminant >= 0.0
    }
}