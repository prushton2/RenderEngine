use crate::ds;
use crate::material;
use crate::object;

pub struct Unified {
    color: ds::Color,
    reflective_pct: u32,
    absorb_pct: u32,
    translucent_pct: u32
}

impl Unified {
    pub fn new(color: u32, reflective_pct: u32, absorb_pct: u32, translucent_pct: u32) -> Self {
        assert!(reflective_pct + absorb_pct + translucent_pct <= 100);
        Self { 
            color: ds::Color::from_u32(color),
            reflective_pct: reflective_pct,
            absorb_pct: absorb_pct,
            translucent_pct: translucent_pct
        }
    }
}

impl material::Material for Unified {
    fn ray_color(&self,
        camera: &object::Camera,
        object: &dyn object::Renderable,
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        ray: &ds::Ray,
        t: f64,
        surface_pos: &ds::Vector3,
        depth: u32
    ) -> ds::Color {
        let translucent = {
            if self.translucent_pct != 0 {
                camera.ray_color(world, &ds::Ray::new(&ray.at(t+0.0000001), &ray.direction), depth) / 100.0 * self.translucent_pct as f64
            } else {
                ds::Color::from_u32(0x00000000) 
            }
        };
        

        let reflective = { 
            if self.reflective_pct != 0 {
                let normal = object.hit_record(ray, t).outward_surface_normal.unit_vector();
                let new_direction = ray.direction - 2.0 * ray.direction.dot(&normal) * normal;
                let new_ray = ds::Ray::new(&ray.at(t-0.000001), &new_direction);
                camera.ray_color(world, &new_ray, depth) / 100.0 * self.reflective_pct as f64
            } else { 
                ds::Color::from_u32(0x00000000) 
            }
        };

        let absorb = {
            if self.absorb_pct != 0 {
                let mut surface_normal = object.hit_record(ray, t).outward_surface_normal;
                if surface_normal.dot(&ray.direction) > 0.0 {
                    surface_normal = -1.0 * surface_normal;
                }
    
                let bounce_origin = surface_pos + surface_normal * 0.001;
                camera.ray_color(world, &ds::Ray::new(&bounce_origin, &surface_normal), depth) / 100.0 * self.absorb_pct as f64
            } else {
                ds::Color::from_u32(0x00000000)
            }
        };

        let color = {
            if self.translucent_pct + self.reflective_pct + self.absorb_pct == 100 {
                ds::Color::from_u32(0x00000000)
            } else {
                self.color / 100.0 * ((100 - self.translucent_pct - self.reflective_pct - self.absorb_pct) as f64)
            }
        };

        return color + absorb + reflective + translucent;

    }
}