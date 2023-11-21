use yew::prelude::*;

use crate::util::{Color, ArrangeX, ArrangeY};

/// # Properties of [Badge]
#[derive(Properties, Clone, PartialEq)]
pub struct BadgeProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Inner components
    #[prop_or_default]
    pub children: Children,

    /// Show badge more rounded as pill
    #[prop_or_default]
    pub pill: bool,

    /// Show badge positioned
    #[prop_or_default]
    pub position: Option<(ArrangeX, ArrangeY)>,

    /// Color style, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Optional text placed before the children
    #[prop_or_default]
    pub text: String,
}

/// # Badge component
/// Used alongside [crate::util::Color] to create Badge components
///
/// See [BadgeProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Badge;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Badge style={Color::Primary}>
///             {"This is a primary badge!"}
///         </Badge>
///     }
/// }
/// ```
#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let mut classes = Classes::new();
    match &props.position {
        Some(position) => {
            classes.push("position-absolute".to_string());
            classes.push(format!("{}", position.0));
            classes.push(format!("{}", position.1));
            classes.push("translate-middle".to_string());
        }
        None => {}
    }
    classes.push("badge");
    if props.pill {
        classes.push("rounded-pill");
    }
    classes.push(format!("bg-{}", props.style));
    if [Color::Warning, Color::Info, Color::Light].contains(&props.style) {
        classes.push("text-dark");
    }
    classes.push(props.class.clone());

    html! {
        <span
            class={classes}
        >
            { &props.text }
            { for props.children.iter() }
        </span>
    }
}
