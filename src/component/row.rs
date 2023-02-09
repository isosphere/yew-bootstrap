use super::Column;
use yew::prelude::*;

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
pub struct Row {}

/// # Properties for [Row]
#[derive(Properties, Clone, PartialEq)]
pub struct RowProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Children of type [crate::component::Column]
    #[prop_or_default]
    pub children: ChildrenWithProps<Column>,
}

impl Component for Row {
    type Message = ();
    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("row");
        classes.push(props.class.clone());

        html! {
            <div
                class={classes}
            >
                { for props.children.iter() }
            </div>
        }
    }
}
