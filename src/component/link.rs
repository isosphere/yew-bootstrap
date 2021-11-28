use yew::prelude::*;
use yewtil::NeqAssign;

use crate::util::Color;

pub struct Link {
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub stretched: bool,

    #[prop_or_default]
    pub style: Option<Color>,

    #[prop_or_default]
    pub text: String,
}

impl Component for Link {
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
        if let Some(style) = self.props.style.clone() {
            classes.push(format!("link-{}", style.to_bootstrap()));
        }
        if self.props.stretched {
            classes.push("stretched-link");
        }
        classes.push(self.props.class.clone());

        html! {
            <a
                class=classes
            >
                { &self.props.text }
                { for self.props.children.iter() }
            </a>
        }
    }
}
