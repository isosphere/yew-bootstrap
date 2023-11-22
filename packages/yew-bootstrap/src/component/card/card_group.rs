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

/// # Card Group component
/// Grouping container for [Card]s. Displays them directly connected, in a grid layout.
///
/// See [CardGroupProps] for a list of properties.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <CardGroup>
///         <Card body=true>{"Card 1"}</Card>
///         <Card body=true>{"Card 2"}</Card>
///         <Card body=true>{"Card 3"}</Card>
///         <Card body=true>{"Card 4"}</Card>
///     </CardGroup>
///   }
/// }
/// ```
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
