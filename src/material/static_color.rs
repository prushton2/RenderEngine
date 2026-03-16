use crate::ds;
use crate::material;
use crate::object;

pub struct StaticColor {
    color: ds::Color,
}

impl StaticColor {
    pub fn new(color: u32) -> Self {
        Self { 
            color: ds::Color::from_u32(color)
        }
    }
}

impl material::Material for StaticColor {
    fn ray_color(&self,
        _camera: &object::Camera,
        _object: &dyn object::Renderable,
        _world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        _ray: &ds::Ray,
        _t: f64,
        _surface_pos: &ds::Vector3,
        _depth: u32
    ) -> ds::Color {
        return self.color
    }
}