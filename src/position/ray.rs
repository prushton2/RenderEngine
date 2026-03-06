use crate::position::Vector3;
use auto_ops::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    pub fn new(origin: &Vector3, direction: &Vector3) -> Self{
        Self{
            origin: origin.clone(),
            direction: direction.clone()
        }
    }

    pub fn clone(&self) -> Self {
        Self{
            origin: self.origin.clone(),
            direction: self.direction.clone()
        }
    }
}

impl_op_ex!(+ |a: &Ray, b: &Ray| -> Ray{
    Ray{
        origin: a.origin + b.origin,
        direction: a.direction + b.direction
    }
});

impl_op_ex!(- |a: &Ray, b: &Ray| -> Ray{
    Ray{
        origin: a.origin - b.origin,
        direction: a.direction - b.direction
    }
});

impl_op_ex!(* |a: &Ray, b: &Ray| -> Ray{
    Ray{
        origin: a.origin * b.origin,
        direction: a.direction * b.direction
    }
});

impl_op_ex!(/ |a: &Ray, b: &Ray| -> Ray{
    Ray{
        origin: a.origin / b.origin,
        direction: a.direction / b.direction
    }
});