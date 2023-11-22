use yew::prelude::*;

use crate::util::Color;

/// # Properties of [Lead]
#[derive(Properties, Clone, PartialEq)]
pub struct LeadProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Inner components
    #[prop_or_default]
    pub children: Children,

    /// Color style, default [Color::Dark]
    #[prop_or(Color::Dark)]
    pub style: Color,

    /// Optional text placed before the children
    #[prop_or_default]
    pub text: String,
}

/// # Lead component
/// Use Lead to make a paragraph stand out.
///
/// See [LeadProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Lead;
/// fn test() -> Html {
///     html!{
///         <Lead>
///             {"This is a lead paragraph. It stands out from regular paragraphs."}
///         </Lead>
///     }
/// }
/// ```
#[function_component]
pub fn Lead(props: &LeadProps) -> Html {
    let mut classes = Classes::new();
    classes.push("lead");
    classes.push(format!("text-{}", props.style));
    classes.push(props.class.clone());

    html! {
        <p class={classes}>
            { &props.text }
            { for props.children.iter() }
        </p>
    }
}
