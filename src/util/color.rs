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

impl Color {
    pub fn to_bootstrap(&self) -> String {
        match self {
            Color::Primary => "primary".to_string(),
            Color::Secondary => "secondary".to_string(),
            Color::Success => "success".to_string(),
            Color::Info => "info".to_string(),
            Color::Warning => "warning".to_string(),
            Color::Danger => "danger".to_string(),
            Color::Light => "light".to_string(),
            Color::Dark => "dark".to_string(),
            Color::Link => "link".to_string(),
        }
    }
}
