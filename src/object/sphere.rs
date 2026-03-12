use crate::ds;
use crate::object::Renderable;
use crate::object::Intersectable;

pub struct Sphere {
    center: ds::Vector3,
    radius: f64,
    bbox: ds::Aabb
}

impl Sphere {
    pub fn new(center: &ds::Vector3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius: radius,
            bbox: ds::Aabb::from_vector3(&(center-radius), &(center+radius))
        }
    }
}

impl Renderable for Sphere {
    fn intersects(&self, ray: &ds::Ray) -> Option<f64> {
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

    fn color(&self, surface_pos: &ds::Vector3) -> u32 {
        let n = (surface_pos - self.center).unit_vector();
        ((n.x*255.0) as u32) << 16 | ((n.y*255.0) as u32) << 8 | ((n.z*-255.0) as u32)
    }
}