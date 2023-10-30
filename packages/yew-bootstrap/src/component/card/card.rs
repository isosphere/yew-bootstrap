use yew::prelude::*;
use crate::util::{Color, TextColor};

#[derive(Properties, Debug, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    children: Children,
    #[prop_or_default]
    bg: Option<Color>,
    #[prop_or_default]
    text: Option<TextColor>,
    #[prop_or_default]
    border: Option<Color>,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let mut classes = Classes::from("card");
    if let Some(color) = &props.bg {
        classes.push(format!("bg-{}", color));
    }
    if let Some(color) = &props.text {
        classes.push(format!("text-{}", color));
    }
    if let Some(color) = &props.border {
        classes.push(format!("border-{}", color));
    }

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}
