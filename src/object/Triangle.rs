use crate::position::Vector3;

pub struct triangle {
    pub pos1: Vector3::vector3,
    pub pos2: Vector3::vector3,
    pub pos3: Vector3::vector3,
}

pub fn new(pos1: Vector3::vector3, pos2: Vector3::vector3, pos3: Vector3::vector3) -> triangle{
    let s = triangle{
        pos1: pos1,
        pos2: pos2,
        pos3: pos3,
    };
    s
}