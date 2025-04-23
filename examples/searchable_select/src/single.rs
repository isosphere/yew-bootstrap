use yew::prelude::*;
use yew_bootstrap::component::{Container, ContainerSize, SearchableSelect, SOption};

/// Properties for [Single]
#[derive(Properties, PartialEq)]
pub struct SingleProps {
    /// List of items
    pub items: Vec<AttrValue>,
}

/// Single searchable select example
///
/// Show how to handle a SearchableSelect to select a single
/// element and get the value
#[function_component]
pub fn Single(props: &SingleProps) -> Html {
    // Get value of selected items. Empty string when none is selected
    let selected = use_state(|| AttrValue::from(""));

    // Create the options from the list of items and indicate which one is
    // selected, if any
    let options: Vec<SOption> = props.items.iter().enumerate().map(|(i, item)| {
        let value = AttrValue::from(i.to_string());
        SOption {
            selected: value == *selected,
            value,
            title: item.clone(),
            ..SOption::default()
        }
    }).collect();

    // Called when an item selection is change. This is called both when an
    // item is selected and an item is deselected.
    let onselectchange = {
        let selected = selected.clone();
        Callback::from(move |(value, to_selected): (AttrValue, bool)| {
            if to_selected {
                selected.set(value)
            }
        })
    };

    let title = (*selected != "").then_some(format!("Selected value: {}", *selected));

    html! {
        <Container class="mt-5" size={ ContainerSize::Small } >
            <h2>{ "Searchable Select Demo - Single selection" }</h2>
            <p>
                { "Try typing in the dropdown and select a fruit. You have multiple possibilities
                   to select an item:" }
            </p>
            <ul>
                <li>{ "Click on an item," }</li>
                <li>{ "When filtering, the first element becomes active and you can press Enter to select it,"}</li>
                <li>{ "Or use arrow keys to select the one you want then press Enter."}</li>
            </ul>
            <p>
                {"When an option is selected, handler "}<code>{ "onselectchange" }</code> {" is called with
                  the value of the item and "}<code>{ "selected" }</code>{ " field saying if it will be selected
                  or not. Beware that the function is called both for elements gettings selecting and loosing selection."}
            </p>
            <p>
                { "When an element is selected, the title shows the index of the element, provided as "}<code>{"value"}</code>
                { " field." }
            </p>

            <SearchableSelect
                options={options}
                {title}
                placeholder="Select an option..."
                onselectchange={onselectchange}
            />
        </Container>
    }
}