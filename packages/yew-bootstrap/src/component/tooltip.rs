//! Implements tooltip suppport.
//!
//! `yew` presumes it has exclusive control of the DOM, which conflicts with the
//! Bootstrap's assumption that it also has exclusive control of the DOM.
//!
//! So, we need to re-implement the Tooltip plugin using `yew`...
//!
//! * <https://github.com/react-bootstrap/react-bootstrap/blob/master/src/Tooltip.tsx>
//! * <https://github.com/twbs/bootstrap/blob/main/js/src/tooltip.js>

use popper_rs::{
    prelude::{use_popper, Modifier, Offset, Options, Placement, Strategy},
    state::ApplyAttributes,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlElement;
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    /// The node which this tooltip is attached to.
    ///
    /// If the `target` can be `disabled`, pass the same value to
    /// [Tooltip's `disabled` property][Self::disabled] to ensure that it will
    /// be automatically hidden even if it had focus on was being hovered.
    pub target: NodeRef,

    /// ID of the tooltip.
    ///
    /// If this is set, [Tooltip] will set the `target`'s `aria-describedby`
    /// attribute whenever it is visible.
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Content of the tooltip.
    #[prop_or_default]
    pub children: Children,

    /// Placement of the tooltip.
    ///
    /// [Popper's website shows all placement options][0].
    ///
    /// [0]: https://popper.js.org/
    #[prop_or_default]
    pub placement: Placement,

    /// Use fade transition when showing or hiding the tooltip.
    #[prop_or_default]
    pub fade: bool,

    /// If `true`, always show the tooltip, regardless of focus state.
    ///
    /// [`disabled = true`][TooltipProps::disabled] overrides this option.
    #[prop_or_default]
    pub show: bool,

    /// Show the tooltip when the [`target`][Self::target] node recieves input
    /// or keyboard focus.
    ///
    /// This defaults to `true`, but [will not trigger on `disabled` elements][0].
    ///
    /// If the [`target`][Self::target] element can be disabled, pass the same
    /// state to this component's [`disabled` property][Self::disabled] to
    /// ensure that the [Tooltip] will be automatically hidden.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or(true)]
    pub trigger_on_focus: bool,

    /// Show the tooltip when the [`target`][Self::target] node has the mouse
    /// cursor hovered over it.
    ///
    /// This defaults to `true`, but [will not trigger on `disabled` elements][0].
    ///
    /// **Note:** this option has no effect on touchscreen devices. Make sure
    /// there are other ways of displaying the tooltip.
    ///
    /// If the [`target`][Self::target] element can be disabled, pass the same
    /// state to this component's [`disabled` property][Self::disabled] to
    /// ensure that the [Tooltip] will be automatically hidden.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or(true)]
    pub trigger_on_hover: bool,

    /// If `true`, always hide the tooltip. *This overrides all other
    /// conditions.*
    ///
    /// The tooltip will remain part of the DOM.
    ///
    /// [Disabled elements don't fire events][0], including `focusout` on a
    /// currently-focused element and `mouseleave` of a currently-hovered
    /// element.
    ///
    /// This property allows you to hide a [Tooltip] which has
    /// [`trigger_on_focus = true`][Self::trigger_on_focus] or
    /// [`trigger_on_hover = true`][Self::trigger_on_hover] whenever the
    /// [`target`][Self::target] is disabled.
    ///
    /// **Warning:** entirely removing the [Tooltip] from the DOM whenever the
    /// [`target`][Self::target] is disabled can cause strange behaviour if the
    /// that tooltip is currently being displayed.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or_default]
    pub disabled: bool,
}

/// [Bootstrap tooltip component][0].
///
/// Bootstrap's tooltips depend on Popper, which presumes control of the DOM.
/// Yew *also* presumes complete control of the DOM, so this can lead to
/// unexpected behaviour whenever it reuses DOM components.
///
/// This has a few differences from Bootstrap's implementation, which are
/// [similar to `react-bootstrap`][2]:
///
/// * Unlike ordinary Bootstrap, `<Tooltip>` is *always* present in the DOM,
///   even when not displayed.
///
///   A `<Tooltip>` needs to remain part of the DOM if it *could* be shown in a
///   component. Use the [`show`][show] and [`disabled`][disabled] properties to
///   control its display.
///
/// * When using a [`target`][target] which could be `disabled`, pass that
///   state [to the `<Tooltip>` as well][disabled].
///
/// * `<Tooltip>` doesn't support the `title` or `data-bs-title` attributes.
///
///   Instead, `<Tooltip>` uses [`children`][], which supports arbitrary HTML.
///
/// * `<Tooltip>` supports [*all* of Popper's placement options][placement],
///   [not just auto/left/right/top/bottom][1].
///
/// * This
///
/// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/
/// [1]: https://getbootstrap.com/docs/5.3/components/tooltips/#directions
/// [2]: https://github.com/react-bootstrap/react-bootstrap/blob/master/src/Tooltip.tsx
/// [placement]: TooltipProps::placement
/// [show]: TooltipProps::show
/// [target]: TooltipProps::target
/// [disabled]: TooltipProps::disabled
/// [children]: TooltipProps::children
#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let tooltip_ref = use_node_ref();

    // Adapted from https://github.com/ctron/popper-rs/blob/main/examples/yew/src/example/basic.rs
    let options = use_memo(props.placement, |placement| Options {
        placement: (*placement).into(),
        modifiers: vec![Modifier::Offset(Offset {
            skidding: 0,
            distance: 8,
        })],
        strategy: Strategy::Absolute,
        ..Default::default()
    });

    let popper = use_popper(props.target.clone(), tooltip_ref.clone(), options).unwrap();

    let focused = use_state_eq(|| false);
    let hovered = use_state_eq(|| false);

    let onshow = {
        let focused = focused.clone();
        let hovered = hovered.clone();
        Callback::from(move |evt_type: String| match evt_type.as_str() {
            "mouseenter" => hovered.set(true),
            "focusin" => focused.set(true),
            _ => {
                return;
            }
        })
    };

    let onhide = {
        let focused = focused.clone();
        let hovered = hovered.clone();
        Callback::from(move |evt_type: String| match evt_type.as_str() {
            "mouseleave" => hovered.set(false),
            "focusout" => focused.set(false),
            _ => {
                return;
            }
        })
    };

    if props.disabled {
        // Whenever this component is disabled, explicitly set our focus and
        // hover state to false.
        focused.set(false);
        hovered.set(false);
    }

    let show = !props.disabled
        && (props.show
            || (*focused && props.trigger_on_focus)
            || (*hovered && props.trigger_on_hover));
    let data_show = show.then(AttrValue::default);

    use_effect_with((show, popper.instance.clone()), |(show, popper)| {
        if *show {
            let popper = popper.clone();

            spawn_local(async move {
                popper.update().await;
            });
        }
    });

    use_effect_with(
        (tooltip_ref.clone(), popper.state.attributes.popper.clone()),
        |(tooltip_ref, attributes)| {
            tooltip_ref.apply_attributes(attributes);
        },
    );

    // Attach event handlers. These are always wired up, just we ignore the
    // result when they're disabled.
    use_effect_with(props.target.clone(), |target_ref| {
        let show_listener = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| {
            onshow.emit(e.type_());
        }));
        let hide_listener = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| {
            onhide.emit(e.type_());
        }));
        let target_elem = target_ref.cast::<HtmlElement>();

        if let Some(target_elem) = &target_elem {
            let _ = target_elem.add_event_listener_with_callback(
                "focusin",
                show_listener.as_ref().unchecked_ref(),
            );
            let _ = target_elem.add_event_listener_with_callback(
                "focusout",
                hide_listener.as_ref().unchecked_ref(),
            );

            let _ = target_elem.add_event_listener_with_callback(
                "mouseenter",
                show_listener.as_ref().unchecked_ref(),
            );
            let _ = target_elem.add_event_listener_with_callback(
                "mouseleave",
                hide_listener.as_ref().unchecked_ref(),
            );
        };

        move || {
            if let Some(target_elem) = target_elem {
                let _ = target_elem.remove_event_listener_with_callback(
                    "focusin",
                    show_listener.as_ref().unchecked_ref(),
                );
                let _ = target_elem.remove_event_listener_with_callback(
                    "focusout",
                    hide_listener.as_ref().unchecked_ref(),
                );
                let _ = target_elem.remove_event_listener_with_callback(
                    "mouseenter",
                    show_listener.as_ref().unchecked_ref(),
                );
                let _ = target_elem.remove_event_listener_with_callback(
                    "mouseleave",
                    hide_listener.as_ref().unchecked_ref(),
                );
            }
            drop(show_listener);
            drop(hide_listener);
        }
    });

    use_effect_with(
        (props.target.clone(), props.id.clone(), show),
        |(target_ref, tooltip_id, show)| {
            let Some(target_elem) = target_ref.cast::<HtmlElement>() else {
                return;
            };

            match (tooltip_id, show) {
                (Some(tooltip_id), true) => {
                    let _ = target_elem.set_attribute("aria-describedby", tooltip_id);
                }
                _ => {
                    let _ = target_elem.remove_attribute("aria-describedby");
                }
            }
        },
    );

    let mut class = classes!["tooltip", "bs-tooltip-auto"];
    if props.fade {
        class.push("fade");
    }
    if show {
        class.push("show");
    }

    let mut popper_style = popper.state.styles.popper.clone();
    // Make sure `<Tooltip>` doesn't interfere with events going to other
    // elements, even when hidden.
    popper_style.insert("pointer-events".to_string(), "none".to_string());

    let tooltip = create_portal(
        html_nested! {
            <div
                ref={&tooltip_ref}
                role="tooltip"
                {class}
                style={&popper_style}
                data-show={&data_show}
                id={props.id.clone()}
            >
                <div
                    class="tooltip-arrow"
                    data-popper-arrow="true"
                    style={&popper.state.styles.arrow}
                />
                <div class="tooltip-inner">
                    { for props.children.iter() }
                </div>
            </div>
        },
        gloo_utils::body().into(),
    );

    html! {
        <>
            {tooltip}
        </>
    }
}
