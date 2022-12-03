use log::*;
use yew::prelude::*;

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

pub struct Container {}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(ContainerSize::ExtraSmall)]
    pub size: ContainerSize,

    #[prop_or_default]
    pub fluid: bool,
}

impl Component for Container {
    type Message = ();
    type Properties = ComponentProps;

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
