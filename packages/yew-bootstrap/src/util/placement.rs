use popper_rs::prelude::Placement as PopperPlacement;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Placement {
    /// Places the child automatically.
    #[default]
    Auto,
    /// Places the child above the parent.
    Top,
    /// Places the child beneath the parent. 
    Bottom,
    /// Places the child to the left of the parent. This is mirrored in RTL.
    Left,
    /// Places the child to the right of the parent. This is mirrored in RTL.
    Right,
}

impl From<Placement> for PopperPlacement {
    fn from(value: Placement) -> Self {
        match value {
            Placement::Auto => PopperPlacement::Auto,
            Placement::Bottom => PopperPlacement::Bottom,
            Placement::Left => PopperPlacement::Left,
            Placement::Right => PopperPlacement::Right,
            Placement::Top => PopperPlacement::Top,
        }
    }
}
