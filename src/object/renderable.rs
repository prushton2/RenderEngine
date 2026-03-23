use crate::ds;
// use crate::material;

pub trait Renderable: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    // fn intersects(&self, ray: &ds::Ray) -> Option<f64>;
    fn hit_record(&self, ray: &ds::Ray, intersection: f64) -> ds::HitRecord;
}

pub trait ToGpu<T> {
    fn to_gpu(&self) -> T;
}