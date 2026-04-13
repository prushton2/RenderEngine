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

impl From<VerticalAnchor> for u32 {
    fn from(a: VerticalAnchor) -> u32 {
        return match a {
            VerticalAnchor::Top => 0,
            VerticalAnchor::Middle => 1,
            VerticalAnchor::Bottom => 2,
        }
    }
}

#[derive(Copy, Clone)]
pub enum HorizontalAnchor {
    Left,
    Center,
    Right
}

impl From<HorizontalAnchor> for u32 {
    fn from(a: HorizontalAnchor) -> u32 {
        return match a {
            HorizontalAnchor::Left => 0,
            HorizontalAnchor::Center => 1,
            HorizontalAnchor::Right => 2,
        }
    }
}