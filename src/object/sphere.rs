use std::cmp;

use crate::ds;
use crate::object::{Renderable, Intersectable};
use crate::object::renderable::ColorType;

pub struct Sphere {
    center: ds::Vector3,
    radius: f64,
    // bbox: ds::Aabb,
    color: ColorType
}

impl Sphere {
    pub fn new(center: &ds::Vector3, radius: f64, color: ColorType) -> Self {
        Self {
            center: center.clone(),
            radius: radius,
            color: color
            // bbox: ds::Aabb::from_vector3(&(center-radius), &(center+radius))
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
            return Some(
                ((h - discriminant.sqrt() ) / a).min((h + discriminant.sqrt() ) / a)
            );
        }
    }

    fn hit_record(&self, ray: &ds::Ray, intersection: f64) -> ds::Hit_Record {
        ds::Hit_Record {
            outward_surface_normal: (ray.at(intersection) - self.center) / self.radius
        }
    }

    fn center(&self) -> ds::Vector3 {
        return self.center;
    }

    fn color(&self, _surface_pos: &ds::Vector3) -> ColorType {
        self.color
    }
}