use crate::position::Vector3;
use crate::object::Square;
use crate::object::Triangle;
// use crate::object::Square

pub struct cube {
    pub squares: [Square::square; 6],
}

pub fn new(pos1: &Vector3::vector3, pos2: &Vector3::vector3) -> cube {    //I hate this but it should work
    cube {
        squares:
        [
            Square::new( //Front face
                &Vector3::new(pos1.x, pos1.y, pos1.z), 
                &Vector3::new(pos1.x, pos2.y, pos1.z), 
                &Vector3::new(pos2.x, pos2.y, pos1.z), 
                &Vector3::new(pos2.x, pos1.y, pos1.z), 
            ),
            Square::new( //Top Face
                &Vector3::new(pos1.x, pos2.y, pos1.z), 
                &Vector3::new(pos1.x, pos2.y, pos2.z), 
                &Vector3::new(pos2.x, pos2.y, pos2.z), 
                &Vector3::new(pos2.x, pos2.y, pos1.z), 
            ),
            Square::new( //Back Face 
                &Vector3::new(pos1.x, pos1.y, pos2.z), 
                &Vector3::new(pos1.x, pos2.y, pos2.z), 
                &Vector3::new(pos2.x, pos2.y, pos2.z), 
                &Vector3::new(pos2.x, pos1.y, pos2.z), 
            ),
            Square::new( //Bottom Face
                &Vector3::new(pos1.x, pos1.y, pos1.z), 
                &Vector3::new(pos1.x, pos1.y, pos2.z), 
                &Vector3::new(pos2.x, pos1.y, pos2.z), 
                &Vector3::new(pos2.x, pos1.y, pos1.z), 
            ),
            Square::new( //Left Face
                &Vector3::new(pos1.x, pos1.y, pos1.z), 
                &Vector3::new(pos1.x, pos1.y, pos2.z), 
                &Vector3::new(pos1.x, pos2.y, pos2.z), 
                &Vector3::new(pos1.x, pos2.y, pos1.z), 
            ),
            Square::new( //Right Face
                &Vector3::new(pos2.x, pos1.y, pos1.z), 
                &Vector3::new(pos2.x, pos1.y, pos2.z), 
                &Vector3::new(pos2.x, pos2.y, pos2.z), 
                &Vector3::new(pos2.x, pos2.y, pos1.z), 
            ) 
        ]
    }

}

impl cube {
    pub fn getTriangles(&self) -> [Triangle::triangle; 12] {
        let mut returnArray: [Triangle::triangle; 12] = [Triangle::triangle{pos1: Vector3::new(0.0, 0.0, 0.0), pos2: Vector3::new(0.0, 0.0, 0.0), pos3: Vector3::new(0.0, 0.0, 0.0)}; 12];
        for i in 0..6 {
            returnArray[i*2+0] = self.squares[i].t1.clone();
            returnArray[i*2+1] = self.squares[i].t2.clone();
        }
        returnArray
    }
}