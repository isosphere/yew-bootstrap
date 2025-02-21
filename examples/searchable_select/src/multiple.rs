use yew::prelude::*;
use yew_bootstrap::component::{Container, ContainerSize, SearchableSelect, SOption};
use std::collections::HashSet;

/// Properties for [Multiple]
#[derive(Properties, PartialEq)]
pub struct MultipleProps {
    /// List of items
    pub items: Vec<AttrValue>,
}

/// Multiple searchable select example
///
/// Show how to handle a SearchableSelect to select a multiple
/// element and get the value
#[function_component]
pub fn Multiple(props: &MultipleProps) -> Html {
    // Hashset of selected items (by value)
    let selected = use_state(HashSet::<usize>::default);

    // Create the options from the list of items and indicate which one is
    // selected, if any
    let options: Vec<SOption> = props.items.iter().enumerate().map(|(i, item)| {
        let value = AttrValue::from(i.to_string());
        SOption {
            selected: (*selected).contains(&i),
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
            let index = value.parse().unwrap();
            let mut set = (*selected).clone();
            if to_selected {
                set.insert(index);
            } else {
                set.remove(&index);
            }
            selected.set(set);
        })
    };

    let selected_list: Vec<String> = (*selected).iter().map(
        |value| value.to_string()
    ).collect();

    let title: Option<String> = (!(*selected).is_empty()).then_some(
        format!("Selected value: {}", selected_list.join(", "))
    );

    html! {
        <Container class="mt-5" size={ ContainerSize::Small } >
            <h2>{ "Searchable Select Demo - Multiple selection" }</h2>
            <p>
                { "Try typing in the dropdown and select a fruit. Multiple selections are possible,
                   with the mouse or by selecting an entry and pressing Enter. "}
            </p>
            <p>
                { "The property "}<code>{"keep_open"}</code>{ " ensures that the dropdown does not
                   close when an item is selected."}
            </p>
            <p>
                {"When an option is selected, handler "}<code>{ "onselectchange" }</code> {" is called with
                  the value of the item and "}<code>{ "selected" }</code>{ " field saying if it will be selected
                  or not. This is used to maintain the list of selected items."}
            </p>

            <SearchableSelect
                options={options}
                {title}
                placeholder="Select one or more option..."
                onselectchange={onselectchange}
                keep_open={true}
            />
        </Container>
    }
}