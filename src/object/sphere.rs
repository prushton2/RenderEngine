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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn center(&self) -> crate::ds::Vector3 {
        self.center
    }

    fn normal(&self, pos: &ds::Vector3) -> ds::Vector3 {
        (pos - self.center).unit_vector()
    }

    fn intersects(&self, ray: &ds::Ray) -> Option<f64> {
        let oc = self.center - ray.origin;
        let a  = ray.direction.dot(&ray.direction);
        let h  = ray.direction.dot(&oc);
        let c  = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (h - discriminant.sqrt()) / a;
        let t2 = (h + discriminant.sqrt()) / a;

        if t1 > 1e-8 { return Some(t1) }
        if t2 > 1e-8 { return Some(t2) }

        return None
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