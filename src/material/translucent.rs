use crate::ds;
use crate::material;
use crate::object;

pub struct Translucent {
    color: ds::Color,
}

impl Translucent {
    pub fn new(color: u32) -> Self {
        Self { 
            color: ds::Color::from_u32(color)
        }
    }
}

impl material::Material for Translucent {
    fn ray_color(&self,
        camera: &object::Camera,
        _object: &dyn object::Renderable,
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        ray: &ds::Ray,
        t: f64,
        _surface_pos: &ds::Vector3,
        depth: u32
    ) -> ds::Color {
        camera.ray_color(world, &ds::Ray::new(&ray.at(t+0.0000001), &ray.direction), depth).blend(self.color)
    }
}