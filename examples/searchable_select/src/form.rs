use yew::prelude::*;
use yew_bootstrap::component::{Container, ContainerSize, SearchableSelect, SOption};
use yew_bootstrap::component::form::{FormControl, FormControlType, FormControlValidation, SelectOption};

/// Properties for [Form]
#[derive(Properties, PartialEq)]
pub struct FormProps {
    /// List of items
    pub items: Vec<AttrValue>,
}

/// Searchable select example in a form
///
/// Show how to handle a SearchableSelect inside a form
#[function_component]
pub fn Form(props: &FormProps) -> Html {
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
            <h2>{ "Searchable Select Demo - Use in a form" }</h2>
            <p>
                { "You can use this component in forms:" }
            </p>
            <ul>
                <li>{ "Set " }<code>{ "id" }</code>{" if needed,"}</li>
                <li>{ "You can set the validation field."}</li>
            </ul>
            <p>{ "Note: some limitations exist, for example floating labels are not available."}</p>

            <FormControl
                id="edit-1"
                ctype={FormControlType::Text}
                class="mb-3"
                label="Text input"
                value="Initial text"
            />
            <SearchableSelect
                id={" searchable-select "}
                options={options}
                {title}
                label={ "Searchable select"}
                placeholder="Select an option..."
                class={ "mb-3" }
                onselectchange={onselectchange}
                validation={
                    match (*selected).as_str() {
                        "" => FormControlValidation::Invalid("You need to select a value".into()),
                        "0" | "1" | "2"  => FormControlValidation::Valid(Some("Selection at the top".into())),
                        _ => FormControlValidation::None,
                    }
                }
            />
            <FormControl
                id="select-1"
                ctype={FormControlType::Select}
                class="mb-3"
                label="Normal select"
                value="Initial text"
                floating={true}
            >
            {
                props.items.iter().enumerate().map(|(i, item)| html! {
                    <SelectOption key={ i } label={ item.clone() } />
                }).collect::<Html>()
            }
            </FormControl>
        </Container>
    }
}