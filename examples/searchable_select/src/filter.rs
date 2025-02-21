use std::rc::Rc;
use yew::prelude::*;
use yew_bootstrap::component::{Container, ContainerSize, SearchableSelect, SOption, filter_case, FilterFn};

/// Properties for [Filter]
#[derive(Properties, PartialEq)]
pub struct FilterProps {
    /// List of items
    pub items: Vec<AttrValue>,
}

/// Searchable select example with custom filter functions
///
/// Show how to customize the filter function in the control
#[function_component]
pub fn Filter(props: &FilterProps) -> Html {
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
            <h2>{ "Searchable Select Demo - Filter example" }</h2>
            <p>
                { "By default filtering uses case-insensitive filtering. You can change to case-sensitive
                   by using the " }<code>{ "filter_case()" }</code>{ "function." }
            </p>

            <SearchableSelect
                options={options.clone()}
                title={title.clone()}
                placeholder="Select an option..."
                filter={ filter_case() }
                onselectchange={onselectchange.clone()}
            />

            <p>
                { "You can also create your own filter. You need to create a function that returns
                   a " }<code>{ "FilterFn" }</code>{ " structure with the wanted functionality.
                   In this case, options are filtered when title starts with provided text (case
                   sensitive)."}
            </p>

            <SearchableSelect
                options={options}
                {title}
                placeholder="Select an option..."
                filter={ filter_custom() }
                onselectchange={onselectchange}
            />
        </Container>
    }
}

fn filter_custom() -> FilterFn {
    FilterFn(Rc::new(
        |value: AttrValue, option: &SOption| {
            option.title.to_string().starts_with(value.as_str())
        }
    ))
}