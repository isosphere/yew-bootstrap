use yew::prelude::*;
use crate::component::card::Card;

/// # Properties of [CardGroup]
#[derive(Properties, Debug, PartialEq)]
pub struct CardGroupProps {
    /// Inner cards (displayed in the [CardGroup])
    #[prop_or_default]
    pub children: ChildrenWithProps<Card>,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn CardGroup(props: &CardGroupProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-group");

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
