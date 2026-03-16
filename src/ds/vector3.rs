use auto_ops::*;
use rand;
use rand::RngExt;

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

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
            z: rng.random_range(min..max),
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let vec = Self::random(-1.0, 1.0);
            let lensq = vec.length_sq();
            if 1e-160 < lensq && lensq <= 1.0 {
                return vec.unit_vector();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) < 0.0 {
            return -1.0 * on_unit_sphere;
        }
        return on_unit_sphere;
    }

    pub fn pitch(&self) -> f64 {
        self.x
    }

    pub fn roll(&self) -> f64 {
        self.y
    }

    pub fn yaw(&self) -> f64 {
        self.z
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