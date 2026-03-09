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
        let a = ray.direction.dot(&ray.direction);
        let b = -2.0*ray.direction.dot(&(self.center - ray.origin));
        let c = (self.center - ray.origin).dot(&(self.center - ray.origin)) - self.radius * self.radius;

        let discriminant = b*b-4.0*a*c;

        if discriminant < 0.0 {
            return None;
        } else {
            return Some((-b - discriminant.sqrt() ) / (2.0*a));
        }
        
        // return discriminant >= 0.0
    }
}