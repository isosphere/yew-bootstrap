use yew::prelude::*;

use crate::util::Color;

pub struct Alert {}

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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("alert");
        classes.push(format!("alert-{}", props.style));
        classes.push(props.class.clone());

        html! {
            <div
                class={classes}
                role="alert"
            >
                { &props.text }
                { for props.children.iter() }
            </div>
        }
    }
}
