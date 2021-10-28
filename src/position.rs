pub mod Vector3 {
    pub struct vector3 {
        pub x: f64,
        pub y: f64, 
        pub z: f64
    }
    pub fn new(x: f64, y: f64, z: f64) -> vector3{
        let s = vector3{x: x, y: y, z: z};
        s
    }

    impl vector3 {
        pub fn add(&self, other: vector3) -> vector3 {
            vector3{
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z
            }
        }
    }
}

pub mod Angle {
    pub struct angle {
        pub angle: f64,
    }
    pub fn new(angle: f64) -> angle {
        let s = angle{angle: angle};
        s
    }
}

pub mod Rotation {
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
}
