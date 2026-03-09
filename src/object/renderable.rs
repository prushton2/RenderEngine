use crate::position;

pub trait Renderable {
    fn intersects(&self, ray: &position::Ray) -> Option<f64>;
}