use std::fmt;

/// # Colors
/// Bootstrap colors for buttons, links, etc.
#[derive(Clone, PartialEq, Eq)]
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

/// # Colors
/// Bootstrap colors for text. Like [Color] but includes white and muted.
#[derive(Clone, PartialEq, Eq)]
pub enum TextColor {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
    Link,
    White,
    Muted,
}

impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextColor::Primary => write!(f, "primary"),
            TextColor::Secondary => write!(f, "secondary"),
            TextColor::Success => write!(f, "success"),
            TextColor::Info => write!(f, "info"),
            TextColor::Warning => write!(f, "warning"),
            TextColor::Danger => write!(f, "danger"),
            TextColor::Light => write!(f, "light"),
            TextColor::Dark => write!(f, "dark"),
            TextColor::Link => write!(f, "link"),
            TextColor::White => write!(f, "white"),
            TextColor::Muted => write!(f, "muted"),
        }
    }
}
