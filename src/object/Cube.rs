use crate::position::Vector3;
use crate::object::Square;
// use crate::object::Square

pub struct cube {
    pub s1: Square::square,
    pub s2: Square::square,
    pub s3: Square::square,
    pub s4: Square::square,
    pub s5: Square::square,
    pub s6: Square::square,
}

pub fn new(pos1: Vector3::vector3, pos2: Vector3::vector3) -> cube{    //I hate this but it should work
    cube {
        s1: Square::new( //Front face
            Vector3::new(pos1.x, pos1.y, pos1.z), 
            Vector3::new(pos1.x, pos2.y, pos1.z), 
            Vector3::new(pos2.x, pos2.y, pos1.z), 
            Vector3::new(pos2.x, pos1.y, pos1.z), 
        ),
        s2: Square::new( //Top Face
            Vector3::new(pos1.x, pos2.y, pos1.z), 
            Vector3::new(pos1.x, pos2.y, pos2.z), 
            Vector3::new(pos2.x, pos2.y, pos2.z), 
            Vector3::new(pos2.x, pos2.y, pos1.z), 
        ),
        s3: Square::new( //Back Face 
            Vector3::new(pos1.x, pos1.y, pos2.z), 
            Vector3::new(pos1.x, pos2.y, pos2.z), 
            Vector3::new(pos2.x, pos2.y, pos2.z), 
            Vector3::new(pos2.x, pos1.y, pos2.z), 
        ),
        s4: Square::new( //Bottom Face
            Vector3::new(pos1.x, pos1.y, pos1.z), 
            Vector3::new(pos1.x, pos1.y, pos2.z), 
            Vector3::new(pos2.x, pos1.y, pos2.z), 
            Vector3::new(pos2.x, pos1.y, pos1.z), 
        ),
        s5: Square::new( //Left Face
            Vector3::new(pos1.x, pos1.y, pos1.z), 
            Vector3::new(pos1.x, pos1.y, pos2.z), 
            Vector3::new(pos1.x, pos2.y, pos2.z), 
            Vector3::new(pos1.x, pos2.y, pos1.z), 
        ),
        s6: Square::new( //Right Face
            Vector3::new(pos2.x, pos1.y, pos1.z), 
            Vector3::new(pos2.x, pos1.y, pos2.z), 
            Vector3::new(pos2.x, pos2.y, pos2.z), 
            Vector3::new(pos2.x, pos2.y, pos1.z), 
        ),
    }
}