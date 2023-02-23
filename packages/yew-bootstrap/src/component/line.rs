use crate::util::*;
use yew::prelude::*;

/// # Horizontal or vertical line
/// 
/// See [LineProps] for a listing of properties.
/// 
/// ## Example
/// Example line:
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Line;
/// use yew_bootstrap::util::{Size, Color};
/// fn test() -> Html {
///     html!{
///         <Line width={ Size::Percent(80.0) } height={ Size::Px(1) } style={ Color::Light }/>
///     }
/// }
/// ```
pub struct Line {}

/// Properties for [Line]
#[derive(Properties, Clone, PartialEq)]
pub struct LineProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Height of the line
    #[prop_or_default]
    pub height: Option<Size>,

    /// Vertical (true) or horizontal line
    #[prop_or_default]
    pub vertical: bool,

    /// Color style
    #[prop_or_default]
    pub style: Option<Color>,

    /// Width of the line
    #[prop_or_default]
    pub width: Option<Size>,
}

impl Component for Line {
    type Message = ();
    type Properties = LineProps;

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
