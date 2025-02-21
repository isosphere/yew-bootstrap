use yew::prelude::*;
use yew_bootstrap::component::{Container, ContainerSize, SearchableSelect, SOption};

/// Properties for [Headers]
#[derive(Properties, PartialEq)]
pub struct HeadersProps {
    /// List of items
    pub items: Vec<AttrValue>,
}

/// Searchable select example with headers
///
/// Shows the possibility to add headers to the list of options
#[function_component]
pub fn Headers(props: &HeadersProps) -> Html {
    // Get value of selected items. Empty string when none is selected
    let selected = use_state(|| AttrValue::from(""));

    // Create the options from the list of items and indicate which one is
    // selected, if any
    let options: Vec<SOption> = props.items.iter().enumerate().map(|(i, item)| {
        let value = AttrValue::from(i.to_string());
        SOption {
            selected: value == *selected,
            header: item.contains("Header"),
            disabled: item.contains("Date"),
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
            <h2>{ "Searchable Select Demo - Headers and disabled" }</h2>
            <p>
                { "Options are divided in headers which appear as dividers. Headers cannot be selected
                   and assumed to start a new group. If a group becomes empty after filtering, it is
                   removed from the list."
                }
            </p>
            <p>
                { "Some entries can also be marked as 'disabled'. These entries cannot be selected either."}
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

