use auto_ops::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64, 
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self{
            x: x, 
            y: y, 
            z: z
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

