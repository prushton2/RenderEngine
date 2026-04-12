use std::any::Any;

use crate::ds;
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

    fn center(&self) -> ds::Vector3 {
        self.q + self.u/2.0 + self.v/2.0
    }

    fn normal(&self, _pos: &ds::Vector3) -> ds::Vector3 {
        return self.normal;
    }

    fn intersects(&self, ray: &ds::Ray) -> Option<f64> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() < 0.00000001 {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denominator;

        if t < 0.0 { return None; }

        let intersection = ray.at(t);
        let planar_hit = intersection - self.q;

        let u_len_sq = self.u.length_sq();
        let v_len_sq = self.v.length_sq();

        let alpha = self.u.dot(&planar_hit) / u_len_sq;
        let beta =  self.v.dot(&planar_hit) / v_len_sq;

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }
        
        return Some(t)
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
