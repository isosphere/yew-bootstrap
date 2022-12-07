use yew::prelude::*;

use crate::util::Color;

/// # Alert component
/// [https://getbootstrap.com/docs/5.1/components/alerts/](https://getbootstrap.com/docs/5.1/components/alerts/)
/// Used alongside [crate::util::Color] to create Alert components 
/// 
/// See [AlertProps] for a listing of properties
/// 
/// ## Example
/// ```rust
/// html!{
///    <Alert style={Color::Primary}>
///     {"This is a primary alert!"}
///    </Alert>
/// }
/// ```
pub struct Alert {}

#[derive(Properties, Clone, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(Color::Primary)]
    pub style: Color,

    #[prop_or_default]
    pub text: String,
}

impl Component for Alert {
    type Message = ();
    type Properties = AlertProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("alert");
        classes.push(format!("alert-{}", props.style));
        classes.push(props.class.clone());

        html! {
            <div
                class={classes}
                role="alert"
            >
                { &props.text }
                { for props.children.iter() }
            </div>
        }
    }
}
