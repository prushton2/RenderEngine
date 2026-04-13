pub struct Anchor {
    pub v_anchor: VerticalAnchor,
    pub h_anchor: HorizontalAnchor
}

impl Anchor {
    pub fn new(v: VerticalAnchor, h: HorizontalAnchor) -> Self {
        Self {
            v_anchor: v,
            h_anchor: h,
        }
    }
}
#[derive(Copy, Clone)]
pub enum VerticalAnchor {
    Top,
    Middle,
    Bottom
}
#[derive(Copy, Clone)]
pub enum HorizontalAnchor {
    Left,
    Center,
    Right
}