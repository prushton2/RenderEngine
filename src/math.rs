use crate::position;
use crate::object;
use std::error;

pub fn getAnglesToPoint(camera: &object::Camera::camera, point: &position::Vector3::vector3) -> position::Rotation::rotation {
    
    let mut rotation = position::Rotation::new(position::Angle::new(0.0), position::Angle::new(0.0), position::Angle::new(0.0));
    
    let a = 1.0;
    let b = getDistance(&position::Vector3::new(camera.pos.x, 0.0, camera.pos.z), &position::Vector3::new(point.x, 0.0, point.z));
    let c = getDistance(&position::Vector3::new(point.x, 0.0, point.z), &position::Vector3::new(camera.pos.x, 0.0, camera.pos.z + 1.0));

    let mut xzangle;

    xzangle = radToDegrees( ((a + b.powf(2.0) - c.powf(2.0)) / (2.0*a*b)).acos() );

    xzangle = if xzangle.is_nan() { 0.0 } else { xzangle };

    let a = 1.0;
    let b = getDistance(&position::Vector3::new(0.0, camera.pos.y, camera.pos.z), &position::Vector3::new(0.0, point.y, point.z));
    let c = getDistance(&position::Vector3::new(0.0, point.y, point.z), &position::Vector3::new(0.0, camera.pos.y, camera.pos.z + 1.0));

    let mut yzangle;

    yzangle = radToDegrees( ((a + b.powf(2.0) - c.powf(2.0)) / (2.0*a*b)).acos() );

    yzangle = if yzangle.is_nan() { 0.0 } else { yzangle };


    rotation.x = position::Angle::new(xzangle);
    rotation.y = position::Angle::new(yzangle);
    
    rotation
}

pub fn getDistance(pos1: &position::Vector3::vector3, pos2: &position::Vector3::vector3) -> f64{
    let sideA = (pos1.x - pos2.x).powf(2.0);
    let sideB = (pos1.y - pos2.y).powf(2.0);
    let sideC = (pos1.z - pos2.z).powf(2.0);
    let sideD = (sideA + sideB + sideC).powf(0.5);
    sideD
}

pub fn radToDegrees(rads: f64) -> f64 {
    180.0/3.14159265358979 * rads
}