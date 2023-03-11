use std::fmt;

/// # Arrange horizontal utility
/// Bootstrap horizontal arranges to position elements.
#[derive(Clone, PartialEq, Eq)]
pub enum ArrangeX {
    Start0,
    Start50,
    Start100,
    End0,
    End50,
    End100,
}

impl fmt::Display for ArrangeX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ArrangeX::Start0 => write!(f, "start-0"),
            ArrangeX::Start50 => write!(f, "start-50"),
            ArrangeX::Start100 => write!(f, "start-100"),
            ArrangeX::End0 => write!(f, "end-0"),
            ArrangeX::End50 => write!(f, "end-50"),
            ArrangeX::End100 => write!(f, "end-100"),
        }
    }
}

/// # Arrange vertical utility
/// Bootstrap vertical arranges to position elements.
#[derive(Clone, PartialEq, Eq)]
pub enum ArrangeY {
    Top0,
    Top50,
    Top100,
    Bottom0,
    Bottom50,
    Bottom100,
}

impl fmt::Display for ArrangeY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ArrangeY::Top0 => write!(f, "top-0"),
            ArrangeY::Top50 => write!(f, "top-50"),
            ArrangeY::Top100 => write!(f, "top-100"),
            ArrangeY::Bottom0 => write!(f, "bottom-0"),
            ArrangeY::Bottom50 => write!(f, "bottom-50"),
            ArrangeY::Bottom100 => write!(f, "bottom-100"),
        }
    }
}