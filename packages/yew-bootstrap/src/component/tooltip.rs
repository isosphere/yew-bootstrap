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
const MEDIA_QUERY_HOVER_NONE: &str = "(hover: none)";

/// Media query to indicate that there is no pointing device which supports
/// hovering.
///
/// Reference: [Media Queries Level 4: All Available Interaction Capabilities](https://www.w3.org/TR/mediaqueries-4/#any-input)
const MEDIA_QUERY_ANY_HOVER_NONE: &str = "(any-hover: none)";

/// Media query to indicate that there are either no pointing devices, or a
/// pointing device only supports coarse input.
///
/// Reference: [Media Queries Level 4: All Available Interaction Capabilities](https://www.w3.org/TR/mediaqueries-4/#any-input)
const MEDIA_QUERY_ANY_POINTER_NONE_OR_COARSE: &str = "(any-pointer: none) or (any-pointer: coarse)";

/// Trigger options for [`TooltipProps::trigger_on_focus`].
///
/// This allows tooltips to be selectively enabled on focus, depending on the
/// result of which [Interaction Media Features][0] the user's device supports.
///
/// [0]: https://www.w3.org/TR/mediaqueries-4/#mf-interaction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipFocusTrigger {
    /// Always show the tooltip on element focus.
    ///
    /// This is the default option, and provides a reliable and accessible
    /// alternative when using a non-hover-capable device (such as a
    /// touchscreen) or navigating with a keyboard on a device that *also* has a
    /// pointing device.
    ///
    /// Because of the many side-effects, browser and platform bugs that come
    /// from attempting to *selectively* disable showing tooltips on focus, this
    /// is generally the best choice, but may lead to unexpected tooltip display
    /// for users on a desktop browser with a traditional mouse.
    #[default]
    Always,

    /// Show the tooltip on element focus only if the *primary* pointing device
    /// does *not* support hovering (eg: touchscreen), or there are no pointing
    /// devices connected (`hover: none`).
    ///
    /// If the primary pointing device supports hovering (eg: mouse, trackpad,
    /// trackball, smart pen, Wiimote, Leap Motion Controller), the tooltip will
    /// not be shown when the element has focus.
    ///
    /// Figuring out what the "primary" pointing device actually is
    /// [can be complicated to answer for some devices][0]. They generally err
    /// towards reporting the use and presence of an ordinary mouse with hover
    /// capabilities (eg: [Firefox bug 1851244][1]), even when there are no
    /// pointing devices connected, or used with a touchscreen.
    ///
    /// Both [Chromium][2] and [Firefox][3] on Windows erroneously report
    /// touch-only devices as having an ordinary mouse with hover capabilities
    /// if the device lacks an auto-rotation sensor (even if disabled), which is
    /// generally the case for non-tablet devices like external touchscreen
    /// monitors and all-in-one PCs).
    ///
    /// [Some Android devices also erroneously report `hover: hover`][4], even
    /// when they only have a touch screen.
    ///
    /// Safari on iOS *always* reports `hover: none`, even when using an iPad
    /// with hover-capable pointing devices, such as the Apple Pencil (stylus),
    /// Magic Keyboard (trackpad) or an ordinary mouse with an external display.
    /// All hover-capable devices *except* the Pencil are reported via
    /// `any-hover: hover`.
    ///
    /// For someone who primarily uses a keyboard to interact with their
    /// computer, but has a mouse plugged in (eg: a laptop with a built-in
    /// trackpad, or a virtual device), their browser will still report a
    /// primary pointing device which is "hover capable", even when they have no
    /// way to hover.
    ///
    /// These implementation problems and shortfalls make the `hover: none`
    /// media query unreliable, and an unreliable indicator of user preferences.
    ///
    /// [0]: https://firefox-source-docs.mozilla.org/widget/windows/windows-pointing-device/index.html#selection-of-the-primary-pointing-device
    /// [1]: https://bugzilla.mozilla.org/show_bug.cgi?id=1851244
    /// [2]: https://issues.chromium.org/issues/366055333
    /// [3]: https://bugzilla.mozilla.org/show_bug.cgi?id=1918292
    /// [4]: https://issues.chromium.org/issues/41445959
    IfHoverNone,

    /// Trigger showing the tooltip on element focus only if *all* pointing
    /// devices connected to the device do not support hovering, or there there
    /// are no pointing devices connected (`any-hover: none`).
    ///
    /// For a device with *only* one non-hovering pointing device (eg: a mobile
    /// phone with a touch screen or basic stylus), this is the same as
    /// [`TooltipFocusTrigger::IfHoverNone`].
    ///
    /// For a device with *both* hovering and non-hovering pointing device(s)
    /// (eg: a laptop with a trackpad and touchscreen, or a tablet with both pen
    /// and touch input), this option will never show will never show the
    /// tooltip on focus.
    ///
    /// Unfortunately, [there is no way to detect if not *all* pointer devices support hovering][1].
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
    ///   (but this will report hover events from touch), due to [Chromium][2]
    ///   and [Firefox][3] bugs.
    ///
    /// These issues may also impact someone who primarily uses a keyboard to
    /// interact with their computer.
    ///
    /// These implementation problems and shortfalls make the `any-hover: none`
    /// media query an unreliable indicator of user preferences.
    ///
    /// [1]: https://github.com/w3c/csswg-drafts/issues/5462
    /// [2]: https://issues.chromium.org/issues/366055333
    /// [3]: https://bugzilla.mozilla.org/show_bug.cgi?id=1918292
    IfAnyHoverNone,

    /// Trigger showing tooltips on element focus only if:
    ///
    /// * there are no pointer devices present (`any-pointer: none`), **or**,
    /// * there are *coarse* pointer devices present (`any-pointer: coarse`),
    ///   such as a touchscreen, Wiimote or Leap Motion Controller
    ///
    /// This is a work-around for there being
    /// [no way for a browser to report that not all devices support `hover`][1],
    /// and the complex heuristics required (which all browsers lack) to
    /// determine which is the "primary" pointing device on desktop and laptop
    /// computers.
    ///
    /// The intent of this mode is that tooltips will be shown on `focus` for
    /// devices with touchscreens, *regardless* of whether they have an
    /// auto-rotation sensor.
    ///
    /// The side-effects are:
    ///
    /// * hovering `coarse` pointer devices (like the Wiimote and Leap Motion
    ///   Controller) will *also* show tooltips on focus, even though they can
    ///   hover
    /// * traditional-style laptops with touchscreens (ie: not foldable or
    ///   convertible into a tablet) will *also* show tooltips on focus, even
    ///   though using the touchscreen as a primary pointing device is very
    ///   uncomfortable (because it requires reaching over the keyboard)
    /// * non-hovering `fine` pointer devices (like basic stylus digitisers)
    ///   will *not* show tooltips on focus, even though they can't hover
    /// * a user primarily using non-pointer (keyboard) input but with at least
    ///   one `fine` pointing device connected (such as a laptop with built-in
    ///   trackpad) will never see tooltips on focus
    /// * [Safari doesn't fire `focus` events][2] on components on click or
    ///   touch if it does not accept keyboard input (eg: `<a>` and `<button>`)
    ///
    /// [1]: https://github.com/w3c/csswg-drafts/issues/5462
    /// [2]: https://webkit.org/b/22261#c68
    IfAnyPointerNoneOrCoarse,

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
            Self::IfHoverNone => MEDIA_QUERY_HOVER_NONE,
            Self::IfAnyHoverNone => MEDIA_QUERY_ANY_HOVER_NONE,
            Self::IfAnyPointerNoneOrCoarse => MEDIA_QUERY_ANY_POINTER_NONE_OR_COARSE,
        };
        let w = gloo_utils::window();
        w.match_media(query).ok().flatten()
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
    /// tooltip on input focus. See [`TooltipFocusTrigger`] for other options
    /// which selectively disable this behaviour based on media queries,
    /// and full caveats on each.
    ///
    /// This [will not trigger on `disabled` elements][0].
    ///
    /// ## Safari/WebKit focus events
    ///
    /// *Unlike* most other web browsers (even on macOS), Safari and other
    /// browsers using the WebKit renderer[^1] do not fire `focus` events for
    /// components which *do not* accept keyboard input (such as `<a>` and
    /// `<button>` elements) when clicked or touched,
    /// [following macOS conventions][2].
    ///
    /// In Safari on macOS, pressing <kbd>Option</kbd> + <kbd>Tab</kbd> will
    /// allow you to focus components that does not accept keyboard input, and
    /// will fire a `focus` event similarly to other platforms. If
    /// [keyboard navigation is enabled in System Settings][3], pressing
    /// <kbd>Tab</kbd> cycles focus between any component (like other
    /// platforms).
    ///
    /// Safari will fire `focus` **and** (potentially-synthetic) `hover` events
    /// for components which accept keyboard input (such as
    /// `<input type="text">`) when clicked *or touched*. Touchscreen devices on
    /// most other platforms will only fire `focus` events on touch, not
    /// `hover`.
    ///
    /// [^1]: [Outside of the European Union][4], **all** browsers on iOS *also* use WebKit.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    /// [2]: https://webkit.org/b/22261#c68
    /// [3]: https://support.apple.com/en-au/guide/mac-help/mchlp1399/14.0/mac/14.0
    /// [4]: https://developer.apple.com/support/alternative-browser-engines/
    #[prop_or_default]
    pub trigger_on_focus: TooltipFocusTrigger,

    /// Show the tooltip when the [`target`][Self::target] component has the
    /// mouse cursor hovered over it.
    ///
    /// This defaults to `true`, but [will not trigger on `disabled` elements][0].
    ///
    /// **Note:** touchscreen devices and keyboard-only users *may not* trigger
    /// hover events. Ensure there is some other way to trigger the tooltip on
    /// those devices, such as with
    /// [`trigger_on_focus={TooltipFocusTrigger::Always}`][1].
    ///
    /// Safari on iOS reports synthetic `mouseenter` events on touchscreen
    /// devices, when browsers on other platforms with touchscreens
    /// traditionally do not.
    ///
    /// However, [Safari on iOS does not fire `focus` events][2] for components
    /// which do not accept keyboard input (such as `<a>` and `<button>`), so
    /// synthetic `hover` events are the only way to trigger tooltips.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    /// [1]: Self::trigger_on_focus
    /// [2]: https://webkit.org/b/22261#c68
    #[prop_or(true)]
    pub trigger_on_hover: bool,

    /// If `true`, always hide the tooltip. *This overrides all other
    /// conditions.*
    ///
    /// The tooltip will remain part of the DOM.
    ///
    /// [Disabled elements don't fire events][0], including `focusout` on a
    /// currently-focused element and `mouseleave` of a currently-hovered
    /// element. This could cause a tooltip to be "stuck" being shown.
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
///   [control display manually][`show`] instead.
///
///   `<Tooltip>` *does not* support the `click` trigger – use input focus
///   instead. This makes it possible to trigger tooltips when there is *no*
///   pointing device available or it cannot be used.
///
/// * Like Bootstrap, tooltips exist in a shadow DOM ([portal][]) outside of the
///   normal page hierarchy.
///
///   Unlike Bootstrap, the `<Tooltip>` is *always* present in the DOM, even
///   when the tooltip is not displayed.
///
///   A `<Tooltip>` needs to remain part of the DOM if it *could* be shown in a
///   component. Use the [`show`][] and [`disabled`][] properties to
///   control its display.
///
/// * When using a [`target`][] which could be `disabled` and triggering
///   on focus and/or on hover, you can prevent the tooltip from being displayed
///   by setting the [`disabled`][] property on the on the `<Tooltip>`
///   as well. Otherwise, the `target` won't fire an event to *hide* the tooltip
///   when it loses focus or isn't hovered.
///
///   If you only ever trigger tooltips manually, then there's no need to sync
///   the `disabled` state.
///
/// * Like Bootstrap, if you want the tooltip to be displayed on focus or on
///   hover on a `disabled` [`target`][], you'll need to use a
///   [wrapper element][].
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
/// [`children`]: TooltipProps::children
/// [`disabled`]: TooltipProps::disabled
/// [portal]: https://yew.rs/docs/advanced-topics/portals
/// [`popper-rs`]: https://github.com/ctron/popper-rs/
/// [`show`]: TooltipProps::show
/// [`target`]: TooltipProps::target
/// [trigger_on_focus]: TooltipProps::trigger_on_focus
/// [trigger_on_hover]: TooltipProps::trigger_on_hover
/// [wrapper element]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let tooltip_ref = use_node_ref();

    // Adapted from https://github.com/ctron/popper-rs/blob/main/examples/yew/src/example/basic.rs
    let options = use_memo(props.placement, |placement| Options {
        placement: *placement,
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
            "mouseenter" => hovered.set(true),
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
