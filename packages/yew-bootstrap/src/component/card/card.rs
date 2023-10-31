use yew::prelude::*;
use crate::util::{Color, TextColor};
use crate::component::card::CardBody;

/// # Properties of [Card]
#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    /// Inner components (displayed in the [Card])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
    /// Color style.
    #[prop_or_default]
    pub bg: Option<Color>,
    /// Text color style.
    #[prop_or_default]
    pub text: Option<TextColor>,
    /// Border color style.
    #[prop_or_default]
    pub border: Option<Color>,
    /// If true, implicitly wraps children in a [CardBody].
    #[prop_or_default]
    pub body: bool,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card");
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
            if props.body {
                <CardBody>{ props.children.clone() }</CardBody>
            } else {
                { props.children.clone() }
            }
        </div>
    }
}
