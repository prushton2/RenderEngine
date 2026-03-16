use crate::ds;
use crate::material;

pub trait Renderable: material::Materialable + Send + Sync {
    fn intersects(&self, ray: &ds::Ray) -> Option<f64>;
    fn hit_record(&self, ray: &ds::Ray, intersection: f64) -> ds::HitRecord;
}