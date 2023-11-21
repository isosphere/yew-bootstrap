use yew::prelude::*;

use crate::util::Color;


/// # Properties of [Alert]
#[derive(Properties, Clone, PartialEq)]
pub struct AlertProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Inner components
    #[prop_or_default]
    pub children: Children,

    /// Color style, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Optional text placed before the children
    #[prop_or_default]
    pub text: String,
}

/// # Alert component
/// Used alongside [crate::util::Color] to create Alert components
///
/// See [AlertProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Alert;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Alert style={Color::Primary}>
///             {"This is a primary alert!"}
///         </Alert>
///     }
/// }
/// ```
#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let mut classes = Classes::new();
    classes.push("alert");
    classes.push(format!("alert-{}", props.style));
    classes.push(props.class.clone());

    html! {
        <div
            class={classes}
            role="alert"
        >
            { &props.text }
            { for props.children.iter() }
        </div>
    }
}
