use yew::prelude::*;
use crate::util::Color;
use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ListGroupVariant {
    Default,
    Flush,
}

impl Default for ListGroupVariant {
    fn default() -> Self {
        ListGroupVariant::Default
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeTrigger {
    AtSize(ContainerSize),
    Always,
    Never,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<ListGroupItem>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: ListGroupVariant,
    #[prop_or_default]
    pub numbered: bool,
    #[prop_or(SizeTrigger::Never)]
    pub horizontal: SizeTrigger,
}

#[function_component]
pub fn ListGroup(props: &ListGroupProps) -> Html {
    let mut classes = Classes::from("list-group");

    match props.variant {
        ListGroupVariant::Default => (),
        ListGroupVariant::Flush => classes.push("list-group-flush"),
    };

    match &props.horizontal {
        SizeTrigger::Never => (),
        SizeTrigger::Always => classes.push("list-group-horizontal"),
        SizeTrigger::AtSize(size) => classes.push(format!("list-group-horizontal-{}", size.to_string())),
    }

    if props.numbered {
        classes.push("list-group-numbered")
    }

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<Color>,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub action: bool,
    #[prop_or_default]
    pub url: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn ListGroupItem(props: &ListGroupItemProps) -> Html {
    let mut classes = Classes::from("list-group-item");
    if props.active {
        classes.push("active");
    }
    if props.disabled {
        classes.push("disabled");
    }
    if let Some(style) = &props.style {
        classes.push(format!("list-group-item-{}", style));
    }

    if props.action && props.url.is_some() {
        classes.push("list-group-item-action");
        html! {
            <a class={classes} href={&props.url} onclick={props.onclick.clone()}>
                {props.children.clone()}
            </a>
        }
    } else if props.action {
        classes.push("list-group-item-action");
        html! {
            <button class={classes} onclick={props.onclick.clone()}>
                {props.children.clone()}
            </button>
        }
    } else {
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}
