use crate::util::*;
use yew::prelude::*;

pub struct Line {}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub height: Option<Size>,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub style: Option<Color>,

    #[prop_or_default]
    pub width: Option<Size>,
}

impl Component for Line {
    type Message = ();
    type Properties = ComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        if props.vertical {
            classes.push("vr");
        }
        if let Some(style) = props.style.clone() {
            classes.push(format!("bg-{}", style));
        }
        classes.push(props.class.clone());

        let mut css = String::new();
        if let Some(height) = props.height.clone() {
            css = format!("height: {}", height);
        }
        if let Some(width) = props.width.clone() {
            css = format!("{}; width: {}", css, width);
        }

        if props.vertical {
            html! {
                <div class={classes} style={css} />
            }
        } else {
            html! {
                <hr class={classes} style={css} />
            }
        }
    }
}
