use yew::prelude::*;

pub struct ButtonGroup {}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub role: String,

    #[prop_or_default]
    pub vertical: bool,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = ComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        if props.vertical {
            classes.push("btn-group-vertical");
        } else {
            classes.push("btn-group");
        }
        classes.push(props.class.clone());

        html! {
            <div
                class={classes}
                role={props.role.clone()}
                aria-label={props.label.clone()}
            >
                { for props.children.iter() }
            </div>
        }
    }
}
