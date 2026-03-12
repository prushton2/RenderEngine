use crate::ds;
use crate::Renderable;

pub struct Quad {
    q: ds::Vector3,
    u: ds::Vector3,
    v: ds::Vector3,
    d: f64,
    normal: ds::Vector3,
    bbox: ds::Aabb
}

impl Quad {
    pub fn new(q: &ds::Vector3, u: &ds::Vector3, v: &ds::Vector3) -> Self {
        Self {
            q: *q,
            u: *u,
            v: *v,
            bbox: ds::Aabb::from_aabb(
                ds::Aabb::from_vector3(q, &(q+u+v)),
                ds::Aabb::from_vector3(&(q+u), &(q+v)),
            ),
            normal: u.cross(&v).unit_vector(),
            d: u.cross(&v).unit_vector().dot(q)
        }
    }
}

impl Renderable for Quad {
    fn intersects(&self, ray: &ds::Ray) -> Option<f64> {
        let denominator = self.normal.dot(&ray.direction);
        
        if denominator.abs() <= 0.00000001 {
            return None
        }

        let t = (self.d - self.normal.dot(&ray.origin))/denominator;

        // Compute the hit point, then express it relative to quad corner q
        let intersection = ray.at(t);
        let planar_hit = intersection - self.q;

        // Project onto u and v axes using their squared lengths
        let u_len_sq = self.u.length_sq();
        let v_len_sq = self.v.length_sq();

        let alpha = self.u.dot(&planar_hit) / u_len_sq;
        let beta  = self.v.dot(&planar_hit) / v_len_sq;

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        return Some(t)
    }


    fn color(&self, surface_pos: &ds::Vector3) -> u32 {
        0x00333333
    }
}