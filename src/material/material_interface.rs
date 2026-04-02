#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuMaterial {
    color:       [f32; 3],
    reflective:  u32,
    translucent: u32,
    _pad0:       u32,
    _pad1:       u32,
    _pad2:       u32,
}

impl GpuMaterial {
    pub fn new(color: u32, reflective_pct: u32, translucent_pct: u32) -> Self {
        assert!(reflective_pct + translucent_pct <= 100);
        Self {
            color: [
                (color >> 16) as f32,
                ((color >> 8) & 255) as f32,
                (color & 255) as f32
            ],
            reflective: reflective_pct,
            translucent: translucent_pct,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        }
    }
}



// use crate::ds;
// use crate::object;

// pub trait Materialable: Send + Sync { // implement this on objects who you want to have a material, and therefore can be rendered
//     fn center(&self) -> ds::Vector3;
//     fn height(&self) -> f64;
//     fn get_material(&self) -> &dyn Material;
// }

// pub trait Material: Send + Sync { // This is the interface materials use to talk to objects and raycasts
//     fn ray_color(&self,
//         camera: &object::Camera, // renderer's camera
//         object: &dyn object::Renderable, // the object the ray intersected with (the owner of this material)
//         world: &Vec<Box<dyn object::Renderable + Send + Sync>>, // all objects in the world
//         ray: &ds::Ray, // the ray that made the intersection
//         t: f64, // distance along the ray that the intersection was made
//         surface_pos: &ds::Vector3, // position on the surface of the object the intersection was made
//         depth: u32 // recursion depth (dont touch, just pass to camera.ray_color if you call it)
//     ) -> ds::Color;
// }