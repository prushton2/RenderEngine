use crate::ds;
use crate::material;
use crate::object;
use crate::ds::Color;

pub struct Molly {
    light: f64,
    dark: f64,
}

impl Molly {
    pub fn new(light: f64, dark: f64) -> Self {
        Self { light, dark }
    }
}

impl material::Material for Molly {
    fn ray_color(&self,
        object: &dyn material::Materialable,
        world: &Vec<Box<dyn object::Renderable + Send + Sync>>,
        ray: &ds::Ray,
        surface_pos: &ds::Vector3,
        depth: u32
    ) -> Color {
        let dist_from_bottom = (surface_pos - object.center()).y + object.height() / 2.0;
        let pct_to_top = 1.0 - (dist_from_bottom / object.height()).clamp(0.0, 1.0);

        // t goes from self.dark to self.light as we move up the object
        let t = self.dark + pct_to_top * (self.light - self.dark);

        let lerp = |a: u8, b: u8| -> u8 {
            (a as f64 + (b as f64 - a as f64) * t) as u8
        };

        // dark pink (bottom) → light pink (top)
        let (dark_r, dark_g, dark_b)   = (0xFF_u8, 0x69_u8, 0x93_u8); // hot pink
        let (light_r, light_g, light_b) = (0xFF_u8, 0xC0_u8, 0xCB_u8); // light pink

        Color::from_u32(
            (lerp(dark_r, light_r) as u32) << 16 |
            (lerp(dark_g, light_g) as u32) << 8  |
            (lerp(dark_b, light_b) as u32)
        )
    }
}