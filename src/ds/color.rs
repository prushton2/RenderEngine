use auto_ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn from_u32(c: u32) -> Self {
        Self {
            r: (c >> 16) as u8,
            g: (c >> 8) as u8,
            b: c as u8
        }
    }

    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
}


impl_op_ex!(+ |a: &Color, b: &Color| -> Color{
    Color{
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b
    }
});

impl_op_ex!(- |a: &Color, b: &Color| -> Color{
    Color{
        r: a.r - b.r,
        g: a.g - b.g,
        b: a.b - b.b
    }
});

impl_op_ex!(* |a: &Color, b: &Color| -> Color{
    Color{
        r: a.r * b.r,
        g: a.g * b.g,
        b: a.b * b.b
    }
});

impl_op_ex!(/ |a: &Color, b: &Color| -> Color{
    Color{
        r: a.r / b.r,
        g: a.g / b.g,
        b: a.b / b.b
    }
});

impl_op_ex_commutative!(+ |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r as u32 + b) as u8,
        g: (a.g as u32 + b) as u8,
        b: (a.b as u32 + b) as u8
    }
});

impl_op_ex_commutative!(- |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r as u32 - b) as u8,
        g: (a.g as u32 - b) as u8,
        b: (a.b as u32 - b) as u8
    }
});

impl_op_ex_commutative!(* |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r as u32 * b) as u8,
        g: (a.g as u32 * b) as u8,
        b: (a.b as u32 * b) as u8
    }
});

impl_op_ex_commutative!(/ |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r as u32 / b) as u8,
        g: (a.g as u32 / b) as u8,
        b: (a.b as u32 / b) as u8
    }
});