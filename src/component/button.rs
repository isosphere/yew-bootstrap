use crate::util::Color;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, PartialEq)]
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

pub struct Button {
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub block: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub outline: bool,

    #[prop_or_default]
    pub size: ButtonSize,

    #[prop_or(Color::Primary)]
    pub style: Color,

    #[prop_or_default]
    pub text: String,
}

impl Component for Button {
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
        classes.push("btn");
        if self.props.outline {
            classes.push(format!("btn-outline-{}", self.props.style.to_bootstrap()));
        } else {
            classes.push(format!("btn-{}", self.props.style.to_bootstrap()));
        }
        match self.props.size {
            ButtonSize::Large => classes.push("btn-lg"),
            ButtonSize::Small => classes.push("btn-sm"),
            _ => (),
        }
        if self.props.block {
            classes.push("btn-block");
        }
        classes.push(self.props.class.clone());

        html! {
            <button
                class=classes
                disabled=self.props.disabled
                name=self.props.name.clone()
                onclick=self.props.onclick.clone()
            >
                { &self.props.text }
                { for self.props.children.iter() }
            </button>
        }
    }
}
