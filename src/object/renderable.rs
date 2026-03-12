use crate::ds;

pub trait Renderable {
    fn intersects(&self, ray: &ds::Ray) -> Option<f64>;
    fn color(&self, surface_pos: &ds::Vector3) -> u32;
}