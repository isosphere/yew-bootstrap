use yew::prelude::*;

/// # Button group
/// [ButtonGroup] is used to group several [crate::component::Button] together.
/// Buttons can be arranged vertically.
/// 
/// Buttons can be grouped in a [ButtonGroup].
/// 
/// See [ButtonGroupProps] for a listing of properties.
/// 
/// ## Example
/// Example of a simple button:
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Button, ButtonGroup};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <ButtonGroup class={ "class" }>
///             <Button style={Color::Primary} text={ "First button" }/>
///             <Button style={Color::Secondary} text={ "Second button" }/>
///         </ButtonGroup>
///     }
/// }
/// ```
pub struct ButtonGroup {}

/// Properties for [ButtonGroup]
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Children for the group
    #[prop_or_default]
    pub children: Children,

    /// Aria label
    #[prop_or_default]
    pub label: String,

    /// Role
    #[prop_or_default]
    pub role: String,

    /// If true, disposition is vertical (Default horizontal)
    #[prop_or_default]
    pub vertical: bool,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = ButtonGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        if props.vertical {
            classes.push("btn-group-vertical");
        } else {
            classes.push("btn-group");
        }
        classes.push(props.class.clone());

        html! {
            <div
                class={classes}
                role={props.role.clone()}
                aria-label={props.label.clone()}
            >
                { for props.children.iter() }
            </div>
        }
    }
}
