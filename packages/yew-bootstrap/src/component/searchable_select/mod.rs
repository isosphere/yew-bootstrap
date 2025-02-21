use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Node, ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition};

use crate::component::form::FormControlValidation;

pub mod soption;
pub mod filter;

pub use soption::*;
pub use filter::*;

/// # Properties for [SearchableSelect]
///
/// See [SearchableSelect] component for the description of the component.
#[derive(Properties, PartialEq)]
pub struct SearchableSelectProps {
    /// List of options to display
    pub options: Vec<SOption>,

    /// Title displayed in the select box. If None, placeholder is
    /// displayed
    pub title: Option<AttrValue>,

    /// Placeholder for the select box when no option is selected
    pub placeholder: AttrValue,

    /// Called when option selection is changed. It is called both when an
    /// item is selected or deselected.
    ///
    /// Args:
    /// - value (AttrValue): value of the option
    /// - selected (bool): indicate if option is going to be selected or not
    pub onselectchange: Callback<(AttrValue, bool)>,

    /// Callback to apply filter when search input is provided.
    /// If None, default filtering is used.
    #[prop_or_else(filter_icase)]
    pub filter: FilterFn,

    /// Keep open when selection changes. This is used to permit multiple
    /// selections
    #[prop_or_default]
    pub keep_open: bool,

    /// Optional class
    #[prop_or_default]
    pub class: Classes,

    /// Disabled if True
    #[prop_or_default]
    pub disabled: bool,

    /// Optional label
    #[prop_or_default]
    pub label: AttrValue,

    /// Optional ID
    #[prop_or_default]
    pub id: AttrValue,

    /// Form validation feedback
    /// Note: you must always validate user input server-side as well, this is only provided for better user experience
    #[prop_or(FormControlValidation::None)]
    pub validation: FormControlValidation,
}


/// # Searchable Select
///
/// This component is similar to a Bootstrap Select components but adds the capability
/// for filtering. Options can be selected using up and down keys followed by Enter.
/// This Select is also compatible with multiple selections.
///
/// See [SearchableSelectProps] for a list of all properties.
///
/// ## Enable the feature
///
/// It requires the `searchable_select` feature:
///
/// ```toml
/// yew-bootstrap = { version="...", features=["searchable_select"] }
/// ```
///
/// ## Options to display
///
/// Options for the Select are not provided as children but as a vector inside the
/// properties, directly setting the fields. For example:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{SearchableSelect, SOption};
///
/// fn test() -> Html {
///     html! {
///         <SearchableSelect
///             options={ vec![
///                 SOption { value: "0".into(), title: "Item 1".into(), selected: true, ..SOption::default() },
///                 SOption { value: "1".into(), title: "Item 2".into(), ..SOption::default() },
///             ]}
///             title=""
///             placeholder="Select an option..."
///             onselectchange={ Callback::default() }
///         />
///     }
/// }
/// ```
///
/// The `placeholder` field show the value displayed in the component when `title` is empty.
///
/// ## Control of the selected value
///
/// Control of the selection is done by:
///
/// - A Callback function called when selection of an item is changed. It is called both when
///   an item is selected and deselected to support multiple selection
/// - The `title` field indicating which elements are selected
/// - Inside the options, the `selected` field to control which option(s) is (are) selected.
///
/// The example components Single and Multiple in examples/searchable_select` show how it can be
/// handled, but below is a simplified example:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{SearchableSelect, SOption};
///
/// #[function_component]
/// fn Test() -> Html {
///     let selected = use_state(|| AttrValue::from(""));
///
///     let options = vec![
///         SOption { value: "0".into(), title: "Item 1".into(), selected: ( *selected == "0" ), ..SOption::default()},
///         SOption { value: "1".into(), title: "Item 2".into(), selected: ( *selected == "1" ), ..SOption::default()},
///     ];
///
///     let onselectchange = {
///         let selected = selected.clone();
///         Callback::from(move |(value, to_selected): (AttrValue, bool)| {
///             if to_selected {
///                 selected.set(value)
///             }
///         })
///     };
///
///     let title = (*selected != "").then_some(format!("Selected value: {}", *selected));
///
///     html! {
///         <SearchableSelect
///             { options }
///             { title }
///             placeholder="Select an option..."
///             onselectchange={ onselectchange.clone() }
///         />
///     }
/// }
/// ```
///
/// ## Use in forms
///
/// The Searchable Select component can be used in forms, and additional fields are available in
/// this purpose:
///
/// - `label` to display a label before the component. Note that floating labels are not supported.
/// - `id` to set an identifier, required when `label` is provided
/// - `validation` field, which accepts a [FormControlValidation] value.
///
/// ## Filtering function
///
/// By default the filtering function filters function in a case insensitive way. The `filter` field
/// can be modified to:
///
/// - Use case-sensitive filtering, using function [filter_case],
/// - Create your own function returning a [FilterFn] structure.
///
/// ## CSS styling
///
/// Some additional styling can be performed:
///
/// - The top level of the component has `searchable_select` class
/// - The list of elements is inside a `list-group` class
/// - Each element including header have class `list-group-item`
/// - Items which are not headers have class `list-group-item-action`, possibly with `selected`, `active` or `disabled` class
/// - Headers elements have class `header`
#[function_component]
pub fn SearchableSelect(props: &SearchableSelectProps) -> Html {
    if props.label != "" && props.id == "" {
        panic!("When a label is provided, an id is required");
    }

    // State for controlling whether the dropdown is visible
    let is_open = use_state(|| false);

    // State for the current search text
    let search_text = use_state(|| AttrValue::from(""));

    // A ref to the outer container, to detect outside clicks
    let container_ref = use_node_ref();

    // A ref to the input entry to set focus when opening the dropbox
    let input_ref = use_node_ref();

    // A ref to the active element to ensure it stays visible
    let active_ref = use_node_ref();

    // Index of the active element when dropbox is open. It is:
    // - set to first selected when dropdown opens, or 0 if none is selected
    // - 0 when search value changes
    //
    // It can be controlled by arrows.
    let active_index = use_state(|| 0);

    let filtered_options = filter_by_group(
        &props.options, (*search_text).clone(), props.filter.clone()
    );

    // Specify active elements, previous and next depending on active_index, and only iterating
    // on elements which are not header and enabled.
    let (
        prev_active, current_active,
        next_active, current_active_index
    ) = get_actives(&filtered_options, *active_index);

    // Toggle the dropdown open/closed
    let on_toggle_dropdown = {
        if props.disabled {
            Callback::from(|_| {})
        } else {
            let is_open = is_open.clone();
            let active_index = active_index.clone();
            let search_text = search_text.clone();

            let first_selected = props.options.iter().position(|option| option.selected);

            Callback::from(move |_| {
                let new_open = !*is_open;
                is_open.set(new_open);
                if new_open {
                    active_index.set(first_selected.unwrap_or(0));
                    search_text.set(AttrValue::from(""));
                }
            })
        }
    };

    // Monitor when is_open or active_index changes to set focus to the search box and ensure
    // that the active element stays visible.
    {
        let is_open = is_open.clone();
        let active_index = active_index.clone();
        let input_ref = input_ref.clone();
        let active_ref = active_ref.clone();
        use_effect_with(
            (*is_open, *active_index),
            move |(is_open, _)| {
                if *is_open {
                    if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                        input.focus().unwrap();
                    }

                    // Make active element visible into the list container, only scrolling this container.
                    if let Some(element) = active_ref.cast::<web_sys::Element>() {
                        let scroll_options = ScrollIntoViewOptions::new();
                        scroll_options.set_block(ScrollLogicalPosition::Nearest);
                        scroll_options.set_behavior(ScrollBehavior::Auto);
                        element.scroll_into_view_with_scroll_into_view_options(&scroll_options);
                    }
                }
            }
        )
    };

    // When the user types in the search box
    let on_search_input = {
        let search_text = search_text.clone();
        let active_index = active_index.clone();
        Callback::from(move |e: InputEvent| {
            // We read the entire value from the event's target
            if let Some(input_elem) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_text.set(input_elem.value().into());
                active_index.set(0);
            }
        })
    };

    // Second handler to specifically detect special keys and move selection
    let on_key_press = {
        let active_index = active_index.clone();
        let is_open = is_open.clone();
        let selectchange = props.onselectchange.clone();

        let (active_value, active_selected) = if let Some(index) = current_active_index {
            let (_, option) = filtered_options.get(index).unwrap();
            (option.value.clone(), option.selected)
        } else {
            (AttrValue::from(""), false)
        };

        let keep_open = props.keep_open;

        Callback::from(move |e: KeyboardEvent| {
            let active_index = active_index.clone();
            match e.key().as_str() {
                "Enter" => {
                    if !keep_open {
                        is_open.set(false);
                    }
                    selectchange.emit((active_value.clone(), !active_selected));
                    e.prevent_default();
                }
                "ArrowUp" => {
                    if let Some(index) = prev_active { active_index.set(index) };
                    e.prevent_default();
                }
                "ArrowDown" => {
                    if let Some(index) = next_active { active_index.set(index) };
                    e.prevent_default();
                },
                "Escape" => {
                    is_open.set(false);
                    e.prevent_default();
                }
                _ => {}
            }
        })
    };

    // When the user clicks on an option in the dropdown
    let onselectchange = {
        let is_open = is_open.clone();
        let selectchange = props.onselectchange.clone();
        let keep_open = props.keep_open;
        Callback::from(move |(value, selected)| {
            if !keep_open {
                is_open.set(false);
            }
            selectchange.emit((value, selected));
        })
    };

    // Detect when clicking outside the dropbox: attach a listener when
    // dropbox is open.
    {
        let is_open = is_open.clone();
        let container_ref = container_ref.clone();

        use_effect_with(
            is_open, // Called each time it changes
            move |is_open| {
                let is_open = is_open.clone();

                let click_closure = {
                    let container_ref = container_ref.clone();
                    let is_open = is_open.clone();

                    Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                        // If user clicked outside container when it is open, close the dropdown
                        if *is_open {
                            if let Some(target) = event.target() {
                                if let Some(container) = container_ref.cast::<Node>() {
                                    if let Ok(target_node) = target.dyn_into::<Node>() {
                                        // if container does NOT contain the clicked node => close
                                        if !container.contains(Some(&target_node)) {
                                            is_open.set(false);
                                        }
                                    }
                                }
                            }
                        }
                    })
                };

                // Assign handler
                let document: Document = gloo_utils::document();
                if *is_open {
                    document
                        .add_event_listener_with_callback("mousedown", click_closure.as_ref().unchecked_ref())
                        .unwrap();
                }

                // Teardown: remove listener on unmounts
                let closure_js = (*is_open).then_some(click_closure.into_js_value());
                let document = document.clone();
                move || {
                    if let Some(closure_js) = closure_js {
                        if let Some(func) = closure_js.dyn_ref::<js_sys::Function>() {
                            let _ = document.remove_event_listener_with_callback("mousedown", func);
                        }
                    }
                }
            }
        )
    };

    let (validation, validation_class) = match props.validation.clone() {
        FormControlValidation::None => (None, None),
        FormControlValidation::Valid(None) => (None, Some("is-valid")),
        FormControlValidation::Valid(Some(text)) => (Some(html! {
            <div class="valid-feedback"> { text.clone() }</div>
        }), Some("is-valid")),
        FormControlValidation::Invalid(text) => (Some(html! {
            <div class="invalid-feedback"> { text.clone() }</div>
        }), Some("is-invalid")),
    };

    let label = (props.label != "").then_some(html! {
        <label for={ props.id.clone() } class={ "form-label" }>{ props.label.clone() }</label>
    });

    html! {
        <div ref={container_ref} class={ classes!("searchable_select", "position-relative", props.class.clone()) }>
            // Select input and toggle button
            { label }
            <div class="input-group" onclick={on_toggle_dropdown.clone()}>
                // Readonly input to show current selection
                <input
                    id={ props.id.clone() }
                    type="text"
                    class={ classes!("form-control", validation_class) }
                    value={ props.title.clone().unwrap_or("".into()) }
                    placeholder={props.placeholder.clone()}
                    disabled={ props.disabled }
                    readonly={true}
                />
                // Toggle button
                <button
                    class="btn btn-outline-secondary dropdown-toggle"
                    type="button"
                    disabled={ props.disabled }
                    onclick={on_toggle_dropdown}
                >
                    { "" }
                </button>
                if ! *is_open {
                    { validation }
                }
            </div>

            // Dropdown area
            if *is_open {
                <div
                    class="position-absolute mt-1 border bg-white w-100 rounded"
                    style="z-index: 9999;"
                >
                    // Search input inside dropdown
                    <div class="p-2">
                        <input
                            ref={input_ref}
                            autofocus={true}
                            type="text"
                            class="form-control"
                            placeholder="Search..."
                            oninput={on_search_input}
                            onkeydown={on_key_press}
                        />
                    </div>

                    // List of filtered options
                    <div class="list-group list-group-flush" style="max-height: 200px; overflow-y: auto;">
                    {
                        filtered_options.iter().map(|(i, option)| {
                            let active = Some(*i) == current_active;
                            html!{
                                <SOptionComp
                                    key={*i}
                                    node_ref={if active { Some(active_ref.clone()) } else { None }}
                                    attrs={(*option).clone()}
                                    onselectchange={onselectchange.clone()}
                                    {active}
                                />
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </div>
            }
        </div>
    }
}

/// Find current active, previous and next, and return filtered index
///
/// options is the result of [super::filter_by_group] and is a vector of `(usize, &SOption)`. This is
/// the list after filtering, but the index (usize) corresponds to the original index before filtering.
///
/// `current_index` corresponds to such index (pre-filter).
///
/// prev: the index (pre-filter) of active element after pressing KeyUp
/// current: the one currently active
/// next: the index (pre-filter) of active element after pressing KeyDown
/// current_index: the index of "current" in the filtered list, independently of the pre-filter index
/// in the element.
fn get_actives(options: &Vec<(usize, &SOption)>, current_index: usize)
    -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>)
{
    let iter = options.iter().filter(
        |(_, option)| !option.disabled && !option.header
    );

    let index = iter.clone().position(|(i, _)| *i >= current_index);

    let indexes: Vec<usize> = iter.map(|(i, _)| *i).collect();

    let current = index.and_then(|index| indexes.get(index).copied() );
    let prev = index.and_then(|index| if index > 0 { indexes.get(index - 1).copied() } else { None } );
    let next = index.and_then(|index| if index < indexes.len() - 1 { indexes.get(index + 1).copied() } else { None });
    let index = current.and_then(|current| options.iter().position(|(i, _)| *i == current));

    (prev, current, next, index)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_get_actives() {
        assert_eq!(
            get_actives(&vec![], 0),
            (None, None, None, None)
        );

        assert_eq!(
            get_actives(&vec![], 1),
            (None, None, None, None)
        );

        let items = vec![
            SOption { title: "Item A0".into(), header: false, ..SOption::default() }, // 0  - 0
            SOption { title: "Group 1".into(), header: true, ..SOption::default() },  // 2  - 1
            SOption { title: "Item B0".into(), header: false, ..SOption::default() }, // 4  - 2
            SOption { title: "Item B1".into(), header: false, ..SOption::default() }, // 6  - 3
            SOption { title: "Group 2".into(), header: true, ..SOption::default() },  // 8  - 4
            SOption { title: "Item C0".into(), header: false, ..SOption::default() }, // 10 - 5
        ];

        // Fake result of filter_by_group, creating an "index" which is twice the normal index
        let options: Vec<(usize, &SOption)> = items.iter().enumerate().map(
            |(i, option)| (i*2, option))
        .collect();

        // Item A0
        assert_eq!(
            get_actives(&options, 0),
            (None, Some(0), Some(4), Some(0))
        );

        // Group 1 -> Item B0
        assert_eq!(
            get_actives(&options, 1),
            (Some(0), Some(4), Some(6), Some(2))
        );

        // Item B1
        assert_eq!(
            get_actives(&options, 6),
            (Some(4), Some(6), Some(10), Some(3))
        );

        // Item C0
        assert_eq!(
            get_actives(&options, 10),
            (Some(6), Some(10), None, Some(5))
        );

        // Too far
        assert_eq!(
            get_actives(&options, 11),
            (None, None, None, None)
        );
    }
}