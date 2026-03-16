use crate::ds;
use crate::object;

pub trait Materialable: Send + Sync { // implement this on objects who you want to have a material, and therefore can be rendered
    fn center(&self) -> ds::Vector3;
    fn height(&self) -> f64;
    fn get_material(&self) -> &dyn Material;
}

pub trait Material: Send + Sync { // This is the interface materials use to talk to objects and raycasts
    fn ray_color(&self,
        camera: &object::Camera, // renderer's camera
        object: &dyn object::Renderable, // the object the ray intersected with (the owner of this material)
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>, // all objects in the world
        ray: &ds::Ray, // the ray that made the intersection
        t: f64, // distance along the ray that the intersection was made
        surface_pos: &ds::Vector3, // position on the surface of the object the intersection was made
        depth: u32 // recursion depth (dont touch, just pass to camera.ray_color if you call it)
    ) -> ds::Color;
}