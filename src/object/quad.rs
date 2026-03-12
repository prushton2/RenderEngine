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

        let numerator = self.d - self.normal.dot(&ray.origin);

        return Some(numerator/denominator)
    }


    fn color(&self, surface_pos: &ds::Vector3) -> u32 {
        0x000000FF
    }
}