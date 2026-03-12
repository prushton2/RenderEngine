use crate::ds;
use crate::Renderable;

pub struct Quad {
    q: ds::Vector3,
    u: ds::Vector3,
    v: ds::Vector3,
    bbox: ds::Aabb
}

impl Quad {
    pub fn new(q: ds::Vector3, u: ds::Vector3, v: ds::Vector3) -> Self {
        Self {
            q: q,
            u: u,
            v: v,
            bbox: ds::Aabb::from_aabb(
                ds::Aabb::from_vector3(&q, &(q+u+v)),
                ds::Aabb::from_vector3(&(q+u), &(q+v)),
            )
        }
    }
}