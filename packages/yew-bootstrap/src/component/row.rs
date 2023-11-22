use super::Column;
use yew::prelude::*;


/// # Properties for [Row]
#[derive(Properties, Clone, PartialEq)]
pub struct RowProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Event called when the element is clicked
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Children of type [crate::component::Column]
    #[prop_or_default]
    pub children: ChildrenWithProps<Column>,
}

/// # Row container
/// Used alongside [crate::component::Column] to create grids
///
/// See [crate::component::RowProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Column, Row};
/// fn test() -> Html {
///     html!{
///         <Row class={"myclass"}>
///             <Column lg=4><p>{ "First column" }</p></Column>
///             <Column lg=8><p>{ "Second column" }</p></Column>
///         </Row>
///     }
/// }
/// ```
#[function_component]
pub fn Row(props: &RowProps) -> Html {
    let mut classes = Classes::new();
    classes.push("row");
    classes.push(props.class.clone());

    html! {
        <div
            class={classes}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}
