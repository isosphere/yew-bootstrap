use yew::prelude::*;
use yewtil::NeqAssign;

use crate::util::Color;

pub struct Alert {
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(Color::Primary)]
    pub style: Color,

    #[prop_or_default]
    pub text: String,
}

impl Component for Alert {
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
        classes.push("alert");
        classes.push(format!("alert-{}", self.props.style));
        classes.push(self.props.class.clone());

        html! {
            <div
                class=classes
                role="alert"
            >
                { &self.props.text }
                { for self.props.children.iter() }
            </div>
        }
    }
}
