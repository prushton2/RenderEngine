use std::any::Any;
use bytemuck;

use crate::ds;
use crate::material::GpuMaterial;
use crate::object::{Renderable, renderable::ToGpu};
// use crate::material::{Materialable, Material};

pub struct Sphere {
    center: ds::Vector3,
    radius: f64,
    // bbox: ds::Aabb,
    material: GpuMaterial
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuSphere {
    center: [f32; 3],
    radius: f32,
    material: GpuMaterial
}

// impl Downcast for Sphere {}

impl Sphere {
    pub fn new(center: &ds::Vector3, radius: f64, material: GpuMaterial) -> Self {
        Self {
            center: center.clone(),
            radius: radius,
            material: material
            // bbox: ds::Aabb::from_vector3(&(center-radius), &(center+radius))
        }
    }
}

impl Renderable for Sphere {
    // fn intersects(&self, ray: &ds::Ray) -> Option<f64> {
    //     let a = ray.direction.length_sq();
    //     let h = ray.direction.dot(&(self.center - ray.origin));
    //     let c = (self.center - ray.origin).length_sq() - self.radius * self.radius;

    //     let discriminant = h*h-a*c;

    //     if discriminant < 0.0 {
    //         return None;
    //     } else {
    //         return Some(
    //             ((h - discriminant.sqrt() ) / a).min((h + discriminant.sqrt() ) / a)
    //         );
    //     }
    // }


    fn as_any(&self) -> &dyn Any {
        self
    }

    fn hit_record(&self, ray: &ds::Ray, intersection: f64) -> ds::HitRecord {
        ds::HitRecord {
            outward_surface_normal: (ray.at(intersection) - self.center) / self.radius
        }
    }
}

impl ToGpu<GpuSphere> for Sphere {
    fn to_gpu(&self) -> GpuSphere {
        GpuSphere {
            center: [self.center.x as f32, self.center.y as f32, self.center.z as f32],
            radius:  self.radius as f32,
            material: self.material,
        }
    }
}

// impl Materialable for Sphere {
//     fn get_material(&self) -> &dyn Material {
//         return self.material.as_ref();
//     }
    
//     fn height(&self) -> f64 {
//         return self.radius * 2.0;
//     }

//     fn center(&self) -> ds::Vector3 {
//         return self.center;
//     }
// }