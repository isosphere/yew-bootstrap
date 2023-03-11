use std::fmt;

/// # Positions
/// Bootstrap positions.
#[derive(Clone, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Position::Static => write!(f, "position-static"),
            Position::Relative => write!(f, "position-relative"),
            Position::Absolute => write!(f, "position-absolute"),
            Position::Fixed => write!(f, "position-fixed"),
            Position::Sticky => write!(f, "position-sticky"),
        }
    }
}
