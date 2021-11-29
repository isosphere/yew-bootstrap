use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Size {
    Auto,
    Initial,
    Inherit,
    Px(u32),
    Em(f32),
    Rem(f32),
    Percent(f32),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Size::Auto => write!(f, "auto"),
            Size::Initial => write!(f, "initial"),
            Size::Inherit => write!(f, "inherit"),
            Size::Px(v) => write!(f, "{}px", v),
            Size::Em(v) => write!(f, "{}em", v),
            Size::Rem(v) => write!(f, "{}rem", v),
            Size::Percent(v) => write!(f, "{}%", v),
        }
    }
}
