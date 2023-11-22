use crate::util::Color;
use yew::prelude::*;

/// # Link component
/// Link component rendered as `<a/>` component. This link can contain
/// any element.
///
/// See [LinkProps] for a listing of properties.
///
/// ## Example
/// Example of link:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Link;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Link style={Color::Primary} stretched={ true } text={ "Link text" } url={ "https://github.com/isosphere/yew-bootstrap/" }/>
///     }
/// }
/// ```
pub struct Link {}

/// Properties for [Link]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Children
    #[prop_or_default]
    pub children: Children,

    /// Stretched if true, making its parent container clickable
    #[prop_or_default]
    pub stretched: bool,

    /// Color style
    #[prop_or_default]
    pub style: Option<Color>,

    /// URL to direct to when the link is clicked
    #[prop_or_default]
    pub url: Option<AttrValue>,

    /// Target frame or window ID for the link
    #[prop_or_default]
    pub target: Option<AttrValue>,

    /// Optional text for the link
    #[prop_or_default]
    pub text: String,
}

impl Component for Link {
    type Message = ();
    type Properties = LinkProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        if let Some(style) = props.style.clone() {
            classes.push(format!("link-{}", style));
        }
        if props.stretched {
            classes.push("stretched-link");
        }
        classes.push(props.class.clone());

        html! {
            <a
                class={classes}
                role={props.url.is_none().then_some("button")}
                href={props.url.clone()}
                target={props.target.clone()}
            >
                { &props.text }
                { for props.children.iter() }
            </a>
        }
    }
}
