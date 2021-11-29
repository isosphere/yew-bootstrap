use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
    Link,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Primary => write!(f, "primary"),
            Color::Secondary => write!(f, "secondary"),
            Color::Success => write!(f, "success"),
            Color::Info => write!(f, "info"),
            Color::Warning => write!(f, "warning"),
            Color::Danger => write!(f, "danger"),
            Color::Light => write!(f, "light"),
            Color::Dark => write!(f, "dark"),
            Color::Link => write!(f, "link"),
        }
    }
}
