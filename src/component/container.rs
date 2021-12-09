use yew::prelude::*;
use yewtil::NeqAssign;
use log::*;

#[derive(Clone, PartialEq)]
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

pub struct Container {
    props: ComponentProps,
}

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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::new();
        // ExtraSmall have no size class
        if self.props.size != ContainerSize::ExtraSmall {
            if self.props.fluid {
                warn!("Fluid is set to true, but a size is also set. Fluid will be ignored.");
            }
            classes.push(format!("container-{}", self.props.size.to_string()));
        } else if self.props.fluid {
            classes.push("container-fluid");
        } else {
            classes.push("container");
        }
        classes.push(self.props.class.clone());

        html! {
            <div
                class=classes
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
