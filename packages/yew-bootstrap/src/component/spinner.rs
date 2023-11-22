use yew::prelude::*;

use crate::util::Color;


/// # Properties of [Spinner]
#[derive(Properties, Clone, PartialEq)]
pub struct SpinnerProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Inner components (visually hidden text)
    #[prop_or_default]
    pub children: Children,

    /// Color style, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Grow style, default false
    #[prop_or_default]
    pub grow: bool,

    /// Small size style, default false
    #[prop_or_default]
    pub small: bool,
}

/// # Spinner component
/// Used alongside [crate::util::Color] to create Spinner components
///
/// See [SpinnerProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Spinner;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Spinner style={Color::Primary}>
///             {"Visually hidden text"}
///         </Spinner>
///     }
/// }
/// ```
#[function_component]
pub fn Spinner(props: &SpinnerProps) -> Html {
    let mut classes = Classes::new();

    if props.grow {
        classes.push("spinner-grow");
        if props.small {
            classes.push("spinner-grow-sm");
        }
    } else {
        classes.push("spinner-border");
        if props.small {
            classes.push("spinner-border-sm");
        }
    }

    classes.push(format!("text-{}", props.style));
    classes.push(props.class.clone());

    html! {
        <div class={classes} role="status">
            <span class="visually-hidden">
                { for props.children.iter() }
            </span>
        </div>
    }
}
