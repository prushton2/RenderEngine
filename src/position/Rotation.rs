use crate::position::Angle;
pub struct rotation {
    pub x: Angle::angle,
    pub y: Angle::angle,
    pub z: Angle::angle,
}
pub fn new(x: Angle::angle, y: Angle::angle, z: Angle::angle) -> rotation {
    let s = rotation{x: x, y: y, z: z};
    s
}

impl rotation {
    pub fn clone(&self) -> rotation {
        rotation {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}