pub struct angle {
    pub angle: f64,
}
pub fn new(angle: f64) -> angle {
    let s = angle{angle: angle};
    s
}

impl angle {
    pub fn add(&self, other: &angle) -> angle {
        angle{
            angle: (self.angle + other.angle) % 360.0
        }
    }
    pub fn sub(&self, other: &angle) -> angle {
        angle{
            angle: (self.angle - other.angle) % 360.0
        }
    }
    pub fn mult(&self, other: &angle) -> angle {
        angle{
            angle: (self.angle * other.angle) % 360.0
        }
    }
    pub fn div(&self, other: &angle) -> angle {
        angle{
            angle: (self.angle / other.angle) % 360.0
        }
    }
}