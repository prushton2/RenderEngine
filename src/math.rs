use crate::position;
use crate::object;

// WHAT WORKS HERE:
// Distance formula

// Unsure:
// Assembled each triangle properly //What does this mean again?

// Doesnt Work
// Calculating the angles
//    - Some issue with negatives, points arent rendering on the other side of the line

pub fn getAnglesToPoint(camera: &object::Camera::camera, point: &position::Vector3::vector3) -> position::Rotation::rotation {
    
    let mut rotation = position::Rotation::new(position::Angle::new(0.0), position::Angle::new(0.0), position::Angle::new(0.0));
    
    // let a = 1.0;
    // let b = getDistance(&position::Vector3::new(camera.pos.x, 0.0, camera.pos.z), &position::Vector3::new(point.x, 0.0, point.z));
    // let c = getDistance(&position::Vector3::new(point.x, 0.0, point.z), &position::Vector3::new(camera.pos.x, 0.0, camera.pos.z + 1.0));

    let a = getDistance(&camera.pos, &position::Vector3::new(point.x, camera.pos.y, point.z));
    let b = getDistance(&position::Vector3::new(point.x, camera.pos.y, point.z), &camera.pos);
    let c = getDistance(&position::Vector3::new(point.x, camera.pos.y, point.z), &position::Vector3::new(camera.pos.x, camera.pos.y, point.z));


    println!("XZ: {}, {}, {}", a, b, c);

    let mut xzangle;

    xzangle = radToDegrees( ((a.powf(2.0) + b.powf(2.0) - c.powf(2.0)) / (2.0*a*b)).acos() );

    xzangle = if xzangle.is_nan() { 0.0 } else { xzangle };

    xzangle = if point.x < camera.pos.x { 360.0 - xzangle } else { xzangle };

    // Point, Player, Point.x, Point.z, Player.y

    let pos1 = point.clone();                                   //  |          |c
    let pos2 = camera.pos.clone();                              //  |a    |b
    let pos3 = position::Vector3::new(pos1.x, pos2.y, pos1.z);  //        |    |

    let a = getDistance(&pos1, &pos2);
    let b = getDistance(&pos2, &pos3);
    let c = getDistance(&pos3, &pos1);

    println!("HEIGHT: {}, {}, {}", a, b, c);

    let mut yzangle;

    yzangle = radToDegrees( ( (a.powf(2.0) + b.powf(2.0) - c.powf(2.0)) / (2.0*a*b) ).acos() );

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