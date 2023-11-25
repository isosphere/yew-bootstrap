use yew::prelude::*;

use crate::util::Color;

/// # Progress component
///
/// See [ProgressProps] for a listing of properties
///
/// # Example of simple Progress bar
/// ``` rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Progress, ProgressBar};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Progress>
///           <ProgressBar style={Some(Color::Primary)} value=25/>
///         </Progress>
///     }
/// }
/// ```
///
/// # Example of multiple bars in the same progress
/// ``` rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Progress, ProgressBar};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Progress>
///             <ProgressBar style={Some(Color::Primary)} value=15/>
///             <ProgressBar style={Some(Color::Info)} value=30/>
///             <ProgressBar style={Some(Color::Warning)} value=20 striped={true}/>
///         </Progress>
///     }
/// }
/// ```

/// Properties for [Progress]
#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub children: Children,

    /// Additional class
    #[prop_or_default]
    pub class: Classes,

    /// Height of the select in pts, default None
    #[prop_or(None)]
    pub height: Option<i32>,
}

/// Properties for [ProgressBar]
#[derive(Properties, Clone, PartialEq)]
pub struct ProgressBarProps {
    /// Optional color for the bar
    #[prop_or_default]
    pub style: Option<Color>,

    /// Current value, between min and max
    pub value: i32,

    /// Minimum value, default 0
    #[prop_or(0)]
    pub min: i32,

    /// Maximum value, default 100
    #[prop_or(100)]
    pub max: i32,

    /// Striped, default false
    #[prop_or_default]
    pub striped: bool,

    /// Animated, this forces striped
    #[prop_or_default]
    pub animated: bool,

    /// Optional label placed on the bar
    #[prop_or_default]
    pub label: AttrValue,

    /// Additional class
    #[prop_or_default]
    pub class: Classes,
}


/// # Progress component, it can contain [ProgressBar] children
#[function_component]
pub fn Progress(props: &ProgressProps) -> Html {
    let height = props.height.map(|val| format!("height: {val}px;"));

    html! {
        <div class={classes!("progress", props.class.clone())} style={height} >
            { for props.children.iter() }
        </div>
    }
}

/// # ProgressBar component, contained inside a [Progress] parent
#[function_component]
pub fn ProgressBar(props: &ProgressBarProps) -> Html {
    if props.value < props.min || props.value > props.max {
        panic!("ProgressBar: value is {}, should be between {} and {}", props.value, props.min, props.max)
    }

    if props.min >= props.max {
        panic!("ProgressBar: min ({}) needs to be less than max ({})", props.min, props.max)
    }

    let width = 100 * (props.value - props.min) / (props.max - props.min);
    let width = format!("width: {width}%;");

    let mut progress_classes = props.class.clone();
    if let Some(color) = &props.style {
        progress_classes.push(format!("bg-{color}"))
    }
    if props.striped || props.animated {
        progress_classes.push("progress-bar-striped")
    }
    if props.animated {
        progress_classes.push("progress-bar-animated")
    }

    html! {
        <div
            class={classes!("progress-bar", progress_classes)}
            role={"progressbar"}
            style={width}
            aria-valuenow={props.value.to_string()}
            aria-valuemin={props.min.to_string()}
            aria-valuemax={props.max.to_string()}
        >
            {props.label.clone()}
        </div>
    }
}