use crate::object::renderable::ToGpu;
use crate::ui::Anchor;
use crate::ui::Image;
use crate::ui::anchor;

pub struct UIElement<'a> {
    image: &'a Image,
    anchor: Anchor
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

impl<'a> UIElement<'a> {
    pub fn new(image: &'a Image, v_anchor: anchor::VerticalAnchor, h_anchor: anchor::HorizontalAnchor) -> UIElement<'a> {
        Self {
            image: image,
            anchor: Anchor::new(v_anchor, h_anchor)
        }
    }

    pub fn get_image(&'a self) -> &'a Image {
        self.image
    }

    pub fn get_anchor(&'a self) -> &'a Anchor {
        &self.anchor
    }
}

impl ToGpu<GPUUIElement> for UIElement<'_> {
    fn to_gpu(&self) -> GPUUIElement {
        GPUUIElement { 
            v_anchor: self.anchor.v_anchor.into(),
            h_anchor: self.anchor.h_anchor.into(),
            width:    self.image.width() as u32,
            height:   self.image.height() as u32, 
            pointer:  0
        }
    }
}