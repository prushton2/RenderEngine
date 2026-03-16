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
        Self {
            light: light,
            dark: dark
        }
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
        let dist_from_bottom: f64 = (surface_pos - object.center()).y + object.height() / 2.0;
        let pct_to_top: f64 = (dist_from_bottom / object.height()).clamp(0.0, 1.0);

        let shade = 1.0/(self.dark + pct_to_top * (self.light - self.dark));
        
        if shade == 0.0 { 
            Color::from_u32(0x00000000) 
        } else { 
            Color::from_u32(0x00FFC0CB) / shade as u32
        }
    }
}