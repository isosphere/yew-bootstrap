use yew::prelude::*;

use crate::util::Color;

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
pub struct Spinner {}

/// # Properties of [Spinner]
#[derive(Properties, Clone, PartialEq)]
pub struct SpinnerProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Inner components
    #[prop_or_default]
    pub children: Children,

    /// Color style, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Grow style, default false
    #[prop_or_default]
    pub grow: bool,

    /// Visually hidden text.
    #[prop_or_default]
    pub text: bool,


    /// Small size style, default false
    #[prop_or_default]
    pub small: bool,
}

impl Component for Spinner {
    type Message = ();
    type Properties = SpinnerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
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
                    {&props.text}
                </span>
                { for props.children.iter() }
            </div>
        }
    }
}
