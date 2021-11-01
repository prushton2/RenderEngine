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
    pub fn clone(&self) -> vector3 {
        vector3{x: self.x, y: self.y, z: self.z}
    }
    pub fn add(&self, other: &vector3) -> vector3 {
        vector3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
    pub fn sub(&self, other: &vector3) -> vector3 {
        vector3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
    pub fn mult(&self, other: &vector3) -> vector3 {
        vector3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
    pub fn div(&self, other: &vector3) -> vector3 {
        vector3{
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
    pub fn eq(&self, other: &vector3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}