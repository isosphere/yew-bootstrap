use yew::prelude::*;


/// Properties for [ButtonGroup]
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Children for the group (Button instances)
    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,

    /// If true, disposition is vertical (Default horizontal)
    #[prop_or_default]
    pub vertical: bool,
}

/// # Button group
/// [ButtonGroup] is used to group several [crate::component::Button] instances together.
/// Buttons can be arranged vertically.
///
/// See [ButtonGroupProps] for a listing of properties.
///
/// ## Example
/// Example of a simple button group:
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
#[function_component]
pub fn ButtonGroup(props: &ButtonGroupProps) -> Html {
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
