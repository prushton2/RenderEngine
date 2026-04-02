pub trait Renderable: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait ToGpu<T> {
    fn to_gpu(&self) -> T;
}