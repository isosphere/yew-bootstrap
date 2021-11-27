use crate::util::Color;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Button {
    props: ButtonProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or(Color::Primary)]
    pub style: Color,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub children: Children,
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
        html! {
            <button
                class={ format!("btn btn-{}", self.props.style.to_bootstrap()) }
                onclick=self.props.onclick.clone()
            >
                { for self.props.children.iter() }
            </button>
        }
    }
}
