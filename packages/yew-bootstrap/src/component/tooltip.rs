//! Implements tooltip suppport.
//!
//! `yew` presumes it has exclusive control of the DOM, which conflicts with the
//! Bootstrap's assumption that it also has exclusive control of the DOM.
//!
//! So, we need to re-implement the Tooltip plugin using `yew`...
//!
//! * <https://github.com/react-bootstrap/react-bootstrap/blob/master/src/Tooltip.tsx>
//! * <https://github.com/twbs/bootstrap/blob/main/js/src/tooltip.js>

pub use popper_rs::prelude::Placement;
use popper_rs::{
    prelude::{use_popper, Modifier, Offset, Options, Strategy},
    state::ApplyAttributes,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlElement, MediaQueryList, MediaQueryListEvent};
use yew::{html::IntoPropValue, platform::spawn_local, prelude::*};

/// Media query to indicate that the primary pointing device is missing or does
/// not support hovering.
///
/// Reference: [Media Queries Level 4: Hover Capability](https://www.w3.org/TR/mediaqueries-4/#hover)
const MEDIA_QUERY_HOVER_NONE: &'static str = "(hover: none)";

/// Media query to indicate that there is no pointing device which supports
/// hovering.
///
/// Reference: [Media Queries Level 4: All Available Interaction Capabilities](https://www.w3.org/TR/mediaqueries-4/#any-input)
const MEDIA_QUERY_ANY_HOVER_NONE: &'static str = "(any-hover: none)";

/// Trigger options for [TooltipProps::trigger_on_focus].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipFocusTrigger {
    /// Always show the tooltip on element focus.
    ///
    /// This is the default option, and provides a reliable alternative when
    /// using a non-hover-capable device (such as a touchscreen) or navigating
    /// with a keyboard.
    #[default]
    Always,

    /// Show the tooltip on element focus only if the primary pointing device
    /// does *not* support hovering (eg: touchscreen), or there are no pointing
    /// devices connected.
    ///
    /// If the primary pointing device supports hovering (eg: mouse, trackpad,
    /// trackball, smart pen, Wiimote), the tooltip will not be shown when
    /// the element has focus.
    ///
    /// On desktop browsers, figuring out what a "primary pointing device"
    /// actually is [can be complicated to answer for some devices][0]. They
    /// generally err towards reporting the use and presence of an ordinary
    /// mouse with hover capabilities (eg: [Firefox bug 1851244][1]), even when
    /// there are no pointing devices connected, or used with a touchscreen.
    ///
    /// **Note:** someone who primarily uses a keyboard to interact with their
    /// computer, but has a mouse plugged in would still have their browser
    /// report a primary pointing device which is "hover capable".
    ///
    /// [0]: https://firefox-source-docs.mozilla.org/widget/windows/windows-pointing-device/index.html#selection-of-the-primary-pointing-device
    /// [1]: https://bugzilla.mozilla.org/show_bug.cgi?id=1851244
    IfNoHover,

    /// Trigger showing the tooltip on element focus only if *all* pointing
    /// devices connected to the device do not support hovering, or there there
    /// are no pointing devices connected.
    ///
    /// For a device with *only* one non-hovering pointing device (eg: a mobile
    /// phone with a touch screen or basic stylus), this is the same as
    /// [`TooltipFocusTrigger::IfNoHover`].
    ///
    /// For a device with *both* hovering and non-hovering pointing device(s)
    /// (eg: a laptop with a trackpad and touchscreen, or a tablet with both pen
    /// and touch input), this will never trigger the tooltip.
    ///
    /// Most desktop browsers will *always* report the presence of an ordinary
    /// (hover-capable) mouse, even if none is attached. This can be caused by:
    ///
    /// * a wireless mouse dongle which is plugged in, but the wireless mouse
    ///   itself is turned off
    ///
    /// * the presence of a PS/2 mouse controller
    ///
    /// * the presence of a virtual mouse device
    ///
    /// * a touch screen which does not have an automatic rotation sensor
    ///   (but this will report hover events from touch)
    ///
    /// These issues may also impact someone who primarily uses a keyboard to
    /// interact with their computer.
    IfNoAnyHover,

    /// Never show the tooltip on element focus.
    ///
    /// Make sure there is some other way to trigger the tooltip which works on
    /// all types of devices and meets users' preferred input modalities.
    Never,
}

impl IntoPropValue<TooltipFocusTrigger> for bool {
    fn into_prop_value(self) -> TooltipFocusTrigger {
        if self {
            TooltipFocusTrigger::Always
        } else {
            TooltipFocusTrigger::Never
        }
    }
}

impl TooltipFocusTrigger {
    fn media_queries(&self) -> Option<MediaQueryList> {
        let query = match self {
            Self::Always | Self::Never => return None,
            Self::IfNoHover => MEDIA_QUERY_HOVER_NONE,
            Self::IfNoAnyHover => MEDIA_QUERY_ANY_HOVER_NONE,
        };
        let w = gloo_utils::window();
        w.match_media(&query).ok().flatten()
    }

    fn should_trigger(&self) -> bool {
        let Some(queries) = self.media_queries() else {
            return match self {
                Self::Always => true,
                Self::Never => false,
                _ => unreachable!(),
            };
        };

        queries.matches()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    /// The node which this tooltip is attached to.
    ///
    /// If the `target` can be `disabled`, pass the same value to
    /// [Tooltip's `disabled` property][Self::disabled] to ensure that the
    /// tooltip will be automatically hidden, even if it had focus, was being
    /// hovered or clicked.
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
    /// This defaults to [`TooltipFocusTrigger::Always`], which always shows the
    /// tooltip on input focus. See [`TooltipFocusTrigger`] for other options,
    /// and full caveats on each.
    ///
    /// This [will not trigger on `disabled` elements][0].
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or_default]
    pub trigger_on_focus: TooltipFocusTrigger,

    /// Show the tooltip when the [`target`][Self::target] node has the mouse
    /// cursor hovered over it.
    ///
    /// This defaults to `true`, but [will not trigger on `disabled` elements][0].
    ///
    /// **Note:** touchscreen devices *may not* trigger hover events. Ensure
    /// there is some other way to trigger the tooltip on those devices, such as
    /// with `trigger_on_focus={TooltipFocusTrigger::IfNoHover}`.
    ///
    /// This will attempt to ignore synthetic `mouseenter` events from
    /// touchscreen devices which report `(any-hover: none)` (iOS). Desktop
    /// browsers with touchscreens may still send hover events from touch.
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
    /// This property allows you to automatically hide a [Tooltip] which has
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

/// # Tooltip component
///
/// Tooltip which is automatically shown when an element is focused or hovered.
///
/// [Bootstrap's tooltips][0] depend on Popper, which assumes complete control
/// of the DOM. Yew *also* assumes complete control of the DOM, so this can lead
/// to unexpected behaviour whenever it reuses DOM components – so you can't
/// just use `data-bs-toggle="tooltip"`.
///
/// This component is similar to [`react-bootstrap`'s Tooltip component][2] –
/// it wires up Popper in a way that works nicely with Yew.
///
/// There are some similarities and differences between this component and
/// Bootstrap's in-built implementation:
///
/// * There's no need to use `bootstrap.bundle.min.js` or `popper.min.js`. This
///   component uses [`popper-rs`][], which comes with Popper.
///
/// * You can't trigger or describe tooltips with `data-bs-*` attributes.
///
/// * The `<Tooltip>`'s content is set with the [`children`][] property, which
///   supports arbitrary HTML.
///
///   It doesn't support the `sanitize`, `sanitizeFn` or `title` attributes.
///
/// * Like Bootstrap, `<Tooltip>` is triggered by
///   [input focus][trigger_on_focus] and [mouse hover][trigger_on_hover] by
///   default.
///
///   These triggers can be individually disabled, and you can
///   [control display manually][show] instead.
///
///   `<Tooltip>` *does not* support the `click` trigger.
///
/// * Like Bootstrap, tooltips exist in a shadow DOM ([portal][]) outside of the
///   normal page hierarchy.
///
///   Unlike Bootstrap, the `<Tooltip>` is *always* present in the DOM, even
///   when the tooltip is not displayed.
///
///   A `<Tooltip>` needs to remain part of the DOM if it *could* be shown in a
///   component. Use the [`show`][show] and [`disabled`][disabled] properties to
///   control its display.
///
/// * When using a [`target`][target] which could be `disabled`, pass that
///   state [to the `<Tooltip>` as well][disabled]. This prevents the tooltip
///   from ever being displayed.
///
///   Otherwise, a tooltip could be "stuck" being shown if it had focus or was
///   hovered at the time it was disabled.
///
///   There's no need to use a wrapper element if the `disabled` state is kept
///   in sync, or the tooltip is *only* triggered manually.
///
/// ## Examples
///
/// Button with a tooltip, shown automatically on focus or hover:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Button, Tooltip};
/// use yew_bootstrap::util::Color;
///
/// fn test() -> Html {
///     let btn_ref = NodeRef::default();
///     html! {
///         <>
///             <Button style={Color::Primary} node_ref={btn_ref.clone()}>
///                 {"Button with tooltip"}
///             </Button>
///             <Tooltip target={btn_ref}>
///                 {"Tooltip for button."}
///             </Tooltip>
///         </>
///     }
/// }
/// ```
///
/// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/
/// [2]: https://github.com/react-bootstrap/react-bootstrap/blob/master/src/Tooltip.tsx
/// [show]: TooltipProps::show
/// [target]: TooltipProps::target
/// [disabled]: TooltipProps::disabled
/// [children]: TooltipProps::children
/// [trigger_on_focus]: TooltipProps::trigger_on_focus
/// [trigger_on_hover]: TooltipProps::trigger_on_hover
/// [show]: TooltipProps::show
/// [portal]: https://yew.rs/docs/advanced-topics/portals
/// [popper-rs]: https://github.com/ctron/popper-rs/
#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let tooltip_ref = use_node_ref();

    // Adapted from https://github.com/ctron/popper-rs/blob/main/examples/yew/src/example/basic.rs
    let options = use_memo(props.placement, |placement| Options {
        placement: (*placement).into(),
        modifiers: vec![Modifier::Offset(Offset {
            skidding: 0,
            distance: 6,
        })],
        strategy: Strategy::Fixed,
        ..Default::default()
    });

    let popper = use_popper(props.target.clone(), tooltip_ref.clone(), options).unwrap();

    let focused = use_state_eq(|| false);
    let focus_should_trigger = use_state_eq(|| props.trigger_on_focus.should_trigger());
    let hovered = use_state_eq(|| false);

    let onshow = {
        let focused = focused.clone();
        let hovered = hovered.clone();
        Callback::from(move |evt_type: String| match evt_type.as_str() {
            "mouseenter" => {
                // Ignore synthetic hover events on devices that don't support
                // hover at all (iOS), to make it work the same as on Android.
                //
                // Desktop versions of Chromium and Firefox *also* send
                // MouseEnter and MouseLeave events from touchscreens, but
                // generally report `hover: hover` and `any-hover: hover`, even
                // when using a touchscreen, or if there is no hover-capable
                // device attached.
                //
                // See the docs at TooltipFocusTrigger::IfNoAnyHover for more
                // detail.
                if let Ok(Some(query)) =
                    gloo_utils::window().match_media(MEDIA_QUERY_ANY_HOVER_NONE)
                {
                    if query.matches() {
                        return;
                    }
                }

                hovered.set(true);
            }
            "focusin" => focused.set(true),
            _ => {}
        })
    };

    let onhide = {
        let focused = focused.clone();
        let hovered = hovered.clone();
        Callback::from(move |evt_type: String| match evt_type.as_str() {
            "mouseleave" => hovered.set(false),
            "focusout" => focused.set(false),
            _ => {}
        })
    };

    let focus_should_trigger_listener = {
        let focus_should_trigger = focus_should_trigger.clone();

        Callback::from(move |v: bool| {
            focus_should_trigger.set(v);
        })
    };

    use_effect_with(props.trigger_on_focus, |trigger_on_focus| {
        let r = if let Some(media_query_list) = trigger_on_focus.media_queries() {
            let media_query_list_listener = Closure::<dyn Fn(MediaQueryListEvent)>::wrap(Box::new(
                move |e: MediaQueryListEvent| {
                    focus_should_trigger_listener.emit(e.matches());
                },
            ));

            let _ = media_query_list.add_event_listener_with_callback(
                "change",
                media_query_list_listener.as_ref().unchecked_ref(),
            );

            Some((media_query_list_listener, media_query_list))
        } else {
            // Current trigger_on_focus rule doesn't need a MediaQueryList change event listener.
            None
        };

        move || {
            if let Some((media_query_list_listener, media_query_list)) = r {
                let _ = media_query_list.remove_event_listener_with_callback(
                    "change",
                    media_query_list_listener.as_ref().unchecked_ref(),
                );

                drop(media_query_list_listener);
            }
        }
    });

    if props.disabled {
        // Whenever this component is disabled, explicitly set our focus and
        // hover state to false.
        focused.set(false);
        hovered.set(false);
    }

    let show = !props.disabled
        && (props.show
            || (*focused && *focus_should_trigger)
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

    create_portal(
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
    )
}
