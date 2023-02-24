use yew::prelude::*;

/// # Properties for a [SelectOptgroup] component.
#[derive(Properties, Clone, PartialEq)]
pub struct SelectOptgroupProps {
    /// Label for the group
    pub label: AttrValue,
    /// Option or OptGroup children
    #[prop_or_default]
    pub children: Children,
    /// Optional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// # Properties for [SelectOption]
#[derive(Properties, Clone, PartialEq)]
pub struct SelectOptionProps {
    /// Visible label
    pub label: AttrValue,
    /// Value for the select when this option is selected
    #[prop_or_default]
    pub value: AttrValue,
    /// When true, this option is selected. Only one should be selected unless
    /// multiple is set for the select
    #[prop_or_default]
    pub selected: bool,
    /// Optional CSS classes
    #[prop_or_default]
    pub class: Classes,
}


/// # Option group
///
/// Use to separate options inside a select. See [SelectOptgroupProps]
/// for a list of properties.
///
/// It can typically be used inside a [crate::form::FormControl] with
/// [crate::form::FormControlType::Select] type
#[function_component]
pub fn SelectOptgroup(props: &SelectOptgroupProps) -> Html {
    html! {
        <optgroup label={ props.label.clone() } class={ props.class.clone() }>
            { for props.children.iter() }
        </optgroup>
    }
}

/// # Select option
///
/// Options inside a select. See [SelectOptionProps] for a list of properties.
///
/// It can typically be used inside a [crate::form::FormControl] with
/// [crate::form::FormControlType::Select] type, or grouped inside a [SelectOptgroup]
#[function_component]
pub fn SelectOption(props: &SelectOptionProps) -> Html {
    html! {
        <option value={ props.value.clone() } class={ props.class.clone() }
                selected={ props.selected }>
            { props.label.clone()}
        </option>
    }
}

