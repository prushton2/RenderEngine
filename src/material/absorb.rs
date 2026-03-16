use crate::ds;
use crate::material;
use crate::object;

pub struct Absorb {
    color: ds::Color,
}

impl Absorb {
    pub fn new(color: u32) -> Self {
        Self { 
            color: ds::Color::from_u32(color)
        }
    }
}

impl material::Material for Absorb {
    fn ray_color(&self,
        camera: &object::Camera,
        object: &dyn object::Renderable,
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        ray: &ds::Ray,
        t: f64,
        surface_pos: &ds::Vector3,
        depth: u32
    ) -> ds::Color {
        let mut surface_normal = object.hit_record(ray, t).outward_surface_normal;
        if surface_normal.dot(&ray.direction) > 0.0 {
            surface_normal = -1.0 * surface_normal;
        }

        let bounce_origin = surface_pos + surface_normal * 0.001;
        let c2 = camera.ray_color(world, &ds::Ray::new(&bounce_origin, &surface_normal), depth);
        c2.blend(self.color)
    }
}