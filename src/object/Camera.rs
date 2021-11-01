use crate::position;
// use crate::object::Square

pub struct camera {
    pub pos: position::Vector3::vector3,
    pub rot: position::Rotation::rotation,
    pub fov: position::Rotation::rotation,
}

pub fn new(pos: position::Vector3::vector3, rot: position::Rotation::rotation, fov: position::Rotation::rotation) -> camera{    //I hate this but it should work
    camera {
        pos: pos,
        rot: rot,
        fov: fov,
    }
}