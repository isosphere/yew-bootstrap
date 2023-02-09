use log::*;
use yew::prelude::*;

/// Size for a container, from extra small to extra large
#[derive(Clone, PartialEq, Eq)]
pub enum ContainerSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    ExtraExtraLarge,
}
impl ToString for ContainerSize {
    fn to_string(&self) -> String {
        match self {
            &ContainerSize::ExtraSmall => "".to_string(),
            ContainerSize::Small => "sm".to_string(),
            ContainerSize::Medium => "md".to_string(),
            ContainerSize::Large => "lg".to_string(),
            ContainerSize::ExtraLarge => "xl".to_string(),
            ContainerSize::ExtraExtraLarge => "xxl".to_string(),
        }
    }
}

/// # Container component
/// Global container for a page.
/// 
/// See [ContainerProps] for a listing of properties.
/// 
/// ## Example
/// Example of container:
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Container, ContainerSize};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Container size={ContainerSize::Large} fluid={ true }/>
///     }
/// }
/// ```
pub struct Container {}

/// Properties for [Container]
#[derive(Properties, Clone, PartialEq)]
pub struct ContainerProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// List of children
    #[prop_or_default]
    pub children: Children,

    /// Size of the container
    #[prop_or(ContainerSize::ExtraSmall)]
    pub size: ContainerSize,

    /// If true, fluid container - Size ignored and must be default.
    #[prop_or_default]
    pub fluid: bool,
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        // ExtraSmall have no size class
        if props.size != ContainerSize::ExtraSmall {
            if props.fluid {
                warn!("Fluid is set to true, but a size is also set. Fluid will be ignored.");
            }
            classes.push(format!("container-{}", props.size.to_string()));
        } else if props.fluid {
            classes.push("container-fluid");
        } else {
            classes.push("container");
        }
        classes.push(props.class.clone());

        html! {
            <div
                class={classes}
            >
                { for props.children.iter() }
            </div>
        }
    }
}
