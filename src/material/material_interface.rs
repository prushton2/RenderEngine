use crate::ds;
use crate::object;

pub trait Materialable: Send + Sync { // implement this on objects who you want to have a material, and therefore can be rendered
    fn center(&self) -> ds::Vector3;
    fn height(&self) -> f64;
    fn get_material(&self) -> &dyn Material;
}

pub trait Material: Send + Sync { // This is the interface materials use to talk to objects and raycasts
    fn ray_color(&self,
        camera: &object::Camera,
        object: &dyn object::Renderable, 
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>, 
        ray: &ds::Ray,
        t: f64,
        surface_pos: &ds::Vector3, 
        depth: u32
    ) -> ds::Color;
}