pub trait Renderable: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn center(&self) -> crate::ds::Vector3;
    fn intersects(&self, ray: &crate::ds::Ray) -> bool;
}

pub trait ToGpu<T> {
    fn to_gpu(&self) -> T;
}