use crate::util::Color;
use yew::prelude::*;

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

pub struct Button {}

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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("btn");
        if props.outline {
            classes.push(format!("btn-outline-{}", props.style));
        } else {
            classes.push(format!("btn-{}", props.style));
        }
        match props.size {
            ButtonSize::Large => classes.push("btn-lg"),
            ButtonSize::Small => classes.push("btn-sm"),
            _ => (),
        }
        if props.block {
            classes.push("btn-block");
        }
        classes.push(props.class.clone());

        html! {
            <button
                class={classes}
                disabled={props.disabled}
                name={props.name.clone()}
                onclick={props.onclick.clone()}
            >
                { &props.text }
                { for props.children.iter() }
            </button>
        }
    }
}
