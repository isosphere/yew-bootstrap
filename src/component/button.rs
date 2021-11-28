use crate::util::Color;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Button {
    props: ButtonProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub outline: bool,

    #[prop_or(Color::Primary)]
    pub style: Color,

    #[prop_or_default]
    pub text: String,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

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
        classes.push("btn");
        if self.props.outline {
            classes.push(format!("btn-outline-{}", self.props.style.to_bootstrap()));
        } else {
            classes.push(format!("btn-{}", self.props.style.to_bootstrap()));
        }
        classes.push(self.props.class.clone());
        html! {
            <button
                class=classes
                onclick=self.props.onclick.clone()
            >
                { &self.props.text }
                { for self.props.children.iter() }
            </button>
        }
    }
}
