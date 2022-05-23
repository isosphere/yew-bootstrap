use yew::prelude::*;

use crate::util::Color;

pub struct Link {}

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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        if let Some(style) = props.style.clone() {
            classes.push(format!("link-{}", style));
        }
        if props.stretched {
            classes.push("stretched-link");
        }
        classes.push(props.class.clone());

        html! {
            <a
                class={classes}
            >
                { &props.text }
                { for props.children.iter() }
            </a>
        }
    }
}
