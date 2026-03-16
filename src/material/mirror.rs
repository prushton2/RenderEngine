use crate::ds;
use crate::material;
use crate::object;

pub struct Mirror {
    color: ds::Color,
}

impl Mirror {
    pub fn new(color: u32) -> Self {
        Self { 
            color: ds::Color::from_u32(color)
        }
    }
}

impl material::Material for Mirror {
    fn ray_color(&self,
        camera: &object::Camera,
        object: &dyn object::Renderable,
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        ray: &ds::Ray,
        t: f64,
        surface_pos: &ds::Vector3,
        depth: u32
    ) -> ds::Color {
        let normal = object.hit_record(ray, t).outward_surface_normal.unit_vector();
        let new_direction = ray.direction - 2.0 * ray.direction.dot(&normal) * normal;
        let new_ray = ds::Ray::new(&ray.at(t-0.000001), &new_direction);

        return camera.ray_color(world, &new_ray, depth).blend(self.color);
    }
}