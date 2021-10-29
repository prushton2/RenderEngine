use crate::position::Vector3;
use crate::object::Triangle;

pub struct square {
    pub t1: Triangle::triangle,
    pub t2: Triangle::triangle,
}

pub fn new(pos1: Vector3::vector3, pos2: Vector3::vector3, pos3: Vector3::vector3, pos4: Vector3::vector3) -> square{
    let s = square {
        t1: Triangle::new(pos1.clone(), pos2.clone(), pos3.clone()),
        t2: Triangle::new(pos1.clone(), pos4.clone(), pos3.clone()),
    };
    s
}