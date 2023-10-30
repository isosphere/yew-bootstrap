use yew::prelude::*;
use crate::util::Color;
use super::*;

/// The variant style of a [ListGroup]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ListGroupVariant {
    /// Default style, with rounded corners and outer borders.
    Default,
    /// Flush style, removes rounding and border.
    Flush,
}

impl Default for ListGroupVariant {
    fn default() -> Self {
        ListGroupVariant::Default
    }
}

/// A size threshold to trigger a property at
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeTrigger {
    /// Trigger at the given size boundary
    AtSize(ContainerSize),
    /// Always active
    Always,
    /// Never active
    Never,
}

/// # Properties of [ListGroup]
#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupProps {
    /// Inner items (displayed in the [ListGroup]).
    #[prop_or_default]
    pub children: ChildrenWithProps<ListGroupItem>,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
    /// Display variant to use, see [ListGroupVariant] for all options.
    #[prop_or_default]
    pub variant: ListGroupVariant,
    /// Whether to number the list.
    #[prop_or_default]
    pub numbered: bool,
    /// Control when the list is displayed horizontally. Always, or at a certain container size.
    #[prop_or(SizeTrigger::Never)]
    pub horizontal: SizeTrigger,
}

/// # ListGroup component
/// A list of items with various properties, including support for numbering, actions, and item
/// colors.
///
/// Items are expected to be instances of [ListGroupItem]
///
/// See [ListGroupProps] for a list of properties.
///
/// ## Example
/// Example of a simple list group:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{ListGroup, ListGroupItem};
/// fn test() -> Html {
///     html! {
///         <ListGroup>
///             <ListGroupItem>{"First"}</ListGroupItem>
///             <ListGroupItem active=true>{"Second"}</ListGroupItem>
///             <ListGroupItem>{"Third"}</ListGroupItem>
///         </ListGroup>
///     }
/// }
///
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

/// # Properties for [ListGroupItem]
#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupItemProps {
    /// Inner components (displayed in the [ListGroupItem]).
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
    /// Optional color to use for the background and border of this item.
    #[prop_or_default]
    pub style: Option<Color>,
    /// Whether this item is the currently active one
    #[prop_or_default]
    pub active: bool,
    /// Whether this item is disabled
    #[prop_or_default]
    pub disabled: bool,
    /// Whether this item is actionable, enables hover and click reactivity.
    #[prop_or_default]
    pub action: bool,
    /// URL to direct to when the list item is clicked
    #[prop_or_default]
    pub url: Option<AttrValue>,
    /// Event called when the list item is clicked
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// # ListGroupItem
/// Used with [ListGroup] to create grouped lists of items.
///
/// See [ListGroupItemProps] for a list of properties.
///
/// ## Example
/// Example of a simple list group:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{ListGroup, ListGroupItem};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html! {
///         <ListGroup>
///             <ListGroupItem style={Color::Light}>{"First"}</ListGroupItem>
///             <ListGroupItem active=true>{"Second"}</ListGroupItem>
///             <ListGroupItem disabled=true>{"Third"}</ListGroupItem>
///         </ListGroup>
///     }
/// }
///
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
