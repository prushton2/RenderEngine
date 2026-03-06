use crate::position;

pub trait hittable {
    pub fn intersects(&self, ray: &position::Ray) -> bool;
}