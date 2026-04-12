pub trait Renderable: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn center(&self) -> crate::ds::Vector3;
    fn intersects(&self, ray: &crate::ds::Ray) -> Option<f64>;
}

pub trait ToGpu<T> {
    fn to_gpu(&self) -> T;
}