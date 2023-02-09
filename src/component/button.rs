use crate::util::Color;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum ButtonSize {
    Large,
    Normal,
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Normal
    }
}

/// # Button component
/// Button with various properties, including a button to open
/// or close a modal dialog [crate::component::Modal].
/// 
/// Buttons can be grouped in a [crate::component::ButtonGroup].
/// 
/// See [ButtonProps] for a listing of properties.
/// 
/// ## Example
/// Example of a simple button:
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Button;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Button style={Color::Primary} text={ "Button text" }/>
///     }
/// }
/// ```
/// 
/// A button can be linked to a [crate::component::Modal] dialog or 
/// close this modal.
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Button;
/// use yew_bootstrap::component::Modal;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html ! {
///         <>
///             <Modal id="ExampleModal">
///                <Button modal_dismiss={true}>{ "Close the modal" }</Button>
///             </Modal>
///             <Button style={Color::Primary} modal_target={ "ExampleModal" }>
///                 { "Open Modal" }
///             </Button>
///         </>
///     }
/// }
/// ```
pub struct Button {}

/// # Properties for [Button]
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Optional children
    #[prop_or_default]
    pub children: Children,

    /// Treat button as block
    #[prop_or_default]
    pub block: bool,

    /// Button is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Name of the component
    #[prop_or_default]
    pub name: String,

    /// Event called when the button is clicked
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Show button as outlined instead of filled
    #[prop_or_default]
    pub outline: bool,

    /// Size of the button
    #[prop_or_default]
    pub size: ButtonSize,

    /// Color of the button, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Text displayed in the button
    #[prop_or_default]
    pub text: String,

    /// if provided, we will set data-bs-toggle and data-bs-target for modal opening
    #[prop_or_default]
    pub modal_target: Option<String>,

    /// true if this button dismisses the modal that contains it
    #[prop_or_default]
    pub modal_dismiss: bool,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("btn");
        if props.outline {
            classes.push(format!("btn-outline-{}", props.style));
        } else {
            classes.push(format!("btn-{}", props.style));
        }
        match props.size {
            ButtonSize::Large => classes.push("btn-lg"),
            ButtonSize::Small => classes.push("btn-sm"),
            _ => (),
        }
        if props.block {
            classes.push("btn-block");
        }
        classes.push(props.class.clone());

        let modal_dismiss = match props.modal_dismiss {
            true => "modal",
            false => "",
        };

        if let Some(target) = &props.modal_target {
            html! {
                <button
                    class={classes}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    onclick={props.onclick.clone()}
                    data-bs-toggle="modal"
                    data-bs-target={format!("#{}",target.clone())}
                >
                    { &props.text }
                    { for props.children.iter() }
                </button>
            }
        } else {
            html! {
                <button
                    class={classes}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    onclick={props.onclick.clone()}
                    data-bs-dismiss={modal_dismiss}
                >
                    { &props.text }
                    { for props.children.iter() }
                </button>
            }
        }

    }
}
