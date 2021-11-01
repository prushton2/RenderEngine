use crate::position;
use crate::object;

pub fn getAnglesToPoint(camera: &object::Camera::camera, point: &position::Vector3::vector3) -> position::Rotation::rotation {
    
    let rotation = position::Rotation::new(position::Angle::new(0.0), position::Angle::new(0.0), position::Angle::new(0.0));
    
    

    rotation
}

pub fn getDistance(pos1: position::Vector3::vector3, pos2: position::Vector3::vector3) -> f64{
    let sideA = (pos1.x - pos2.x).powf(2.0);
    let sideB = (pos1.y - pos2.y).powf(2.0);
    let sideC = (pos1.z - pos2.z).powf(2.0);
    let sideD = (sideA + sideB + sideC).powf(0.5);
    sideD
}