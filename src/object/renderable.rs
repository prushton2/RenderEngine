use crate::ds;

pub trait Renderable: Send + Sync {
    fn intersects(&self, ray: &ds::Ray) -> Option<f64>;
    fn hit_record(&self, ray: &ds::Ray, intersection: f64) -> ds::Hit_Record;
    fn color(&self, surface_pos: &ds::Vector3) -> ColorType;
}

pub trait Intersectable {
    fn intersects(&self, ray: &ds::Ray) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum ColorType {
    rgb(ds::Color),
    diffuse(ds::Color),
    translucent(ds::Color)
}