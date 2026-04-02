use std::any::Any;

use crate::{ds, material};
use crate::material::GpuMaterial;
use crate::object::{Renderable, renderable::ToGpu};
// use crate::material::{Materialable, Material};

pub struct Quad {
    q: ds::Vector3,
    u: ds::Vector3,
    v: ds::Vector3,
    d: f64,
    normal: ds::Vector3,
    // bbox: ds::Aabb
    material: GpuMaterial
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuQuad {
    q:       [f32; 3],
    _pad0:   f32,
    u:       [f32; 3],
    _pad1:   f32,
    v:       [f32; 3],
    d:       f32,
    normal:  [f32; 3],
    _pad2:   f32,
    material: GpuMaterial
}

impl Quad {
    pub fn new(q: &ds::Vector3, u: &ds::Vector3, v: &ds::Vector3, material: GpuMaterial) -> Self {
        Self {
            q: *q,
            u: *u,
            v: *v,
            // bbox: ds::Aabb::from_aabb(
            //     ds::Aabb::from_vector3(q, &(q+u+v)),
            //     ds::Aabb::from_vector3(&(q+u), &(q+v)),
            // ),
            normal: u.cross(&v).unit_vector(),
            d: u.cross(&v).unit_vector().dot(q), // scalar distance to origin
            material: material
        }
    }
}

impl Renderable for Quad {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn hit_record(&self, _ray: &ds::Ray, _intersection: f64) -> ds::HitRecord {
        ds::HitRecord {
            outward_surface_normal: self.normal
        }
    }
}

impl ToGpu<GpuQuad> for Quad {
    fn to_gpu(&self) -> GpuQuad {
        GpuQuad {
            q:      [self.q.x as f32, self.q.y as f32, self.q.z as f32],
            _pad0:  0.0,
            u:      [self.u.x as f32, self.u.y as f32, self.u.z as f32],
            _pad1:  0.0,
            v:      [self.v.x as f32, self.v.y as f32, self.v.z as f32],
            d:      self.d as f32,
            normal: [self.normal.x as f32, self.normal.y as f32, self.normal.z as f32],
            _pad2:  0.0,
            material: self.material
        }
    }
}


// impl Materialable for Quad {
//     fn get_material(&self) -> &dyn Material {
//         return self.material.as_ref();
//     }
    
//     fn height(&self) -> f64 {
//         return self.v.length();
//     }

//     fn center(&self) -> ds::Vector3 {
//         return self.q + self.v/2.0 + self.u/2.0;
//     }
// }