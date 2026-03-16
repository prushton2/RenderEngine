pub mod material_interface;
pub mod molly;
pub mod static_color;
pub mod absorb;
pub mod translucent;

pub use material_interface::Materialable;
pub use material_interface::Material;
pub use molly::Molly;
pub use static_color::StaticColor;
pub use absorb::Absorb;
pub use translucent::Translucent;