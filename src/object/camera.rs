use crate::position;

pub struct Camera {
    pub pos: position::Vector3,
    pub dir: position::Ray,
}

impl Camera {
    pub fn new(pos: position::Vector3, dir: position::Ray) -> Self{
        Self {
            pos: pos,
            dir: dir,
        }
    }
}