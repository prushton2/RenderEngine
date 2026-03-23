use auto_ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn from_u32(c: u32) -> Self {
        Self {
            r: (c >> 16) as f64,
            g: ((c >> 8) & 255) as f64,
            b: (c & 255) as f64
        }
    }

    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    pub fn blend(&self, b: Color) -> Color {
        Color {
            r: ((self.r + b.r) / 2.0),
            g: ((self.g + b.g) / 2.0),
            b: ((self.b + b.b) / 2.0),
        }
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
        r: (a.r + *b as f64),
        g: (a.g + *b as f64),
        b: (a.b + *b as f64)
    }
});

impl_op_ex_commutative!(- |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r - *b as f64),
        g: (a.g - *b as f64),
        b: (a.b - *b as f64)
    }
});

impl_op_ex_commutative!(* |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r * *b as f64),
        g: (a.g * *b as f64),
        b: (a.b * *b as f64)
    }
});

impl_op_ex_commutative!(/ |a: &Color, b: &u32| -> Color{
    Color{
        r: (a.r / *b as f64),
        g: (a.g / *b as f64),
        b: (a.b / *b as f64)
    }
});

impl_op_ex_commutative!(/ |a: &Color, b: &f64| -> Color{
    Color{
        r: (a.r / b),
        g: (a.g / b),
        b: (a.b / b)
    }
});

impl_op_ex_commutative!(* |a: &Color, b: &f64| -> Color{
    Color{
        r: (a.r * b),
        g: (a.g * b),
        b: (a.b * b)
    }
});