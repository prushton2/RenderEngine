use crate::position;

pub trait Renderable {
    fn intersects(&self, ray: &position::Ray) -> Option<f64>;
    fn color(&self, surface_pos: &position::Vector3) -> u32;
}