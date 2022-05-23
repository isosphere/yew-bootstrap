use super::Column;
use yew::prelude::*;

pub struct Row {}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: ChildrenWithProps<Column>,
}

impl Component for Row {
    type Message = ();
    type Properties = ComponentProps;

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
