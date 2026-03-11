use auto_ops::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64, 
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{
            x: x, 
            y: y, 
            z: z
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        let product = self * other;
        product.x + product.y + product.z
    }

    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn unit_vector(&self) -> Vector3 {
        self / self.length()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3{
    Vector3{
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
});

impl_op_ex!(- |a: &Vector3, b: &Vector3| -> Vector3{
    Vector3{
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z
    }
});

impl_op_ex!(* |a: &Vector3, b: &Vector3| -> Vector3{
    Vector3{
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z
    }
});

impl_op_ex!(/ |a: &Vector3, b: &Vector3| -> Vector3{
    Vector3{
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z
    }
});

impl_op_ex_commutative!(+ |a: &Vector3, b: &f64| -> Vector3{
    Vector3{
        x: a.x + b,
        y: a.y + b,
        z: a.z + b
    }
});

impl_op_ex_commutative!(- |a: &Vector3, b: &f64| -> Vector3{
    Vector3{
        x: a.x - b,
        y: a.y - b,
        z: a.z - b
    }
});

impl_op_ex_commutative!(* |a: &Vector3, b: &f64| -> Vector3{
    Vector3{
        x: a.x * b,
        y: a.y * b,
        z: a.z * b
    }
});

impl_op_ex_commutative!(/ |a: &Vector3, b: &f64| -> Vector3{
    Vector3{
        x: a.x / b,
        y: a.y / b,
        z: a.z / b
    }
});

impl_op!(- |a: &Vector3| -> Vector3 { 
    Vector3{ 
        x: -a.x,
        y: -a.y,
        z: -a.z
    }
});