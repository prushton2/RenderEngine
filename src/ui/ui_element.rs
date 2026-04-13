use crate::object::renderable::ToGpu;
use crate::ui::Anchor;
use crate::ui::Image;
use crate::ui::anchor;

pub struct UIElement {
    image: Image,
    anchor: Anchor,
    gpu_offset: Option<u32>
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUUIElement {
    pub v_anchor: u32,
    pub h_anchor: u32,
    pub width:    u32,
    pub height:   u32,
    pub pointer:  u32
}

impl<'a> UIElement {
    pub fn new(image: Image, v_anchor: anchor::VerticalAnchor, h_anchor: anchor::HorizontalAnchor) -> UIElement {
        Self {
            image: image,
            anchor: Anchor::new(v_anchor, h_anchor),
            gpu_offset: None
        }
    }

    pub fn get_image(&self) -> &Image {
        &self.image
    }

    pub fn get_anchor(&'a self) -> &'a Anchor {
        &self.anchor
    }

    pub fn set_pointer(&mut self, p: Option<u32>) {
        self.gpu_offset = p;
    }
}

impl ToGpu<GPUUIElement> for UIElement {
    fn to_gpu(&self) -> GPUUIElement {
        GPUUIElement { 
            v_anchor: self.anchor.v_anchor.into(),
            h_anchor: self.anchor.h_anchor.into(),
            width:    self.image.width() as u32,
            height:   self.image.height() as u32, 
            pointer:  if self.gpu_offset.is_none() { 0 } else { self.gpu_offset.unwrap() }
        }
    }
}