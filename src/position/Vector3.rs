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