use std::fmt;

use yew::prelude::*;

use crate::util::Color;

/// # Display heading component
/// Use Display when you need heading element to stand out
///
/// See [DisplayProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Display;
/// fn test() -> Html {
///     html!{
///         <Display>
///             {"Display heading text"}
///         </Display>
///     }
/// }
/// ```
pub struct Display {}

/// # Properties of [Alert]
#[derive(Properties, Clone, PartialEq)]
pub struct DisplayProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Inner components
    #[prop_or_default]
    pub children: Children,

    /// Color style, default [Color::Dark]
    #[prop_or(Color::Dark)]
    pub style: Color,

    /// Display size, default [DisplaySize::One]
    #[prop_or(DisplaySize::One)]
    pub size: DisplaySize,

    /// Optional text placed before the children
    #[prop_or_default]
    pub text: String,
}

impl Component for Display {
    type Message = ();
    type Properties = DisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push(format!("display-{}", props.size));
        classes.push(format!("text-{}", props.style));
        classes.push(props.class.clone());

        html! {
            <h1 class={classes}>
                { &props.text }
                { for props.children.iter() }
            </h1>
        }
    }
}

/// # Display sizes
/// Bootstrap display sizes.
#[derive(Clone, PartialEq, Eq)]
pub enum DisplaySize {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl fmt::Display for DisplaySize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplaySize::One => write!(f, "1"),
            DisplaySize::Two => write!(f, "2"),
            DisplaySize::Three => write!(f, "3"),
            DisplaySize::Four => write!(f, "4"),
            DisplaySize::Five => write!(f, "5"),
            DisplaySize::Six => write!(f, "6"),
        }
    }
}

