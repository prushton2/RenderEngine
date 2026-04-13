use crate::ui::Anchor;
use crate::ui::Image;
use crate::ui::anchor;

pub struct UIElement<'a> {
    image: &'a Image,
    anchor: Anchor
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