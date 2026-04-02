#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuMaterial {
    color:       [f32; 3],
    reflective:  u32,
    translucent: u32,
    _pad0:       u32,
    _pad1:       u32,
    _pad2:       u32,
}

impl GpuMaterial {
    pub fn new(color: u32, reflective_pct: u32, translucent_pct: u32) -> Self {
        assert!(reflective_pct + translucent_pct <= 100);
        Self {
            color: [
                (color >> 16) as f32,
                ((color >> 8) & 255) as f32,
                (color & 255) as f32
            ],
            reflective: reflective_pct,
            translucent: translucent_pct,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        }
    }
}