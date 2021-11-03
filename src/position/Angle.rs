pub struct angle {
    pub angle: f64,
}
pub fn new(angle: f64) -> angle {
    let s = angle{angle: angle};
    s
}

impl angle {
    pub fn clone(&self) -> angle {
        angle{angle:self.angle}
    }
    pub fn add(&self, other: &angle) -> angle {
        let mut result = self.angle + other.angle;
        while(result < 0.0) {
            result += 360.0
        }
        angle{
            angle: result % 360.0
        }
    }
    pub fn sub(&self, other: &angle) -> angle {
        let mut result = self.angle - other.angle;
        while(result < 0.0) {
            result += 360.0
        }
        angle{
            angle: result % 360.0
        }
    }
    pub fn mult(&self, other: &angle) -> angle {
        let mut result = self.angle * other.angle;
        while(result < 0.0) {
            result += 360.0
        }
        angle{
            angle: result % 360.0
        }
    }
    pub fn div(&self, other: &angle) -> angle {
        let mut result = self.angle / other.angle;
        while(result < 0.0) {
            result += 360.0
        }
        angle{
            angle: result % 360.0
        }
    }
    pub fn gt(&self, other: &angle) -> bool {
        let angle = self.angle;
        let opposite = self.sub(&new(180.0));
        let opposite = opposite.angle;
        
        if angle - opposite <= 0.0 { 
            other.angle > opposite || other.angle < angle
        }
        else {
            other.angle > opposite && other.angle < angle
        }
    }
    pub fn lt(&self, other: &angle) -> bool {
        let angle = self.angle;
        let opposite = self.sub(&new(180.0));
        let opposite = opposite.angle;
        
        if opposite - angle <= 0.0 { 
            other.angle < opposite || other.angle > angle
        }
        else {
            other.angle < opposite && other.angle > angle
        }
    }
}