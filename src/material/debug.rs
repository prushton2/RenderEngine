use crate::ds;
use crate::material;
use crate::object;

pub struct Debug {}

impl Debug {
    pub fn new() -> Self {
        Self {}
    }
}

impl material::Material for Debug {
    fn ray_color(&self,
        _camera: &object::Camera,
        object: &dyn object::Renderable,
        _world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        _ray: &ds::Ray,
        _t: f64,
        surface_pos: &ds::Vector3,
        _depth: u32
    ) -> ds::Color {
        let n = (surface_pos - object.center()).unit_vector();
        ds::Color::from_u32(((n.x*255.0) as u32) << 16 | ((n.y*255.0) as u32) << 8 | ((n.z*-255.0) as u32))
    }
}