use yew::prelude::*;

/// SVG checkmark to show selected option without depending on external library
const CHECKMARK: &str =
    "data:image/svg+xml,\
    %3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 512 512'%3E%3Cpath fill='currentColor' \
    d='M173.898 439.404l-166.4-166.4c-8.188-8.188-8.188-21.518 0-29.704l29.704-29.704 \
       136.695 136.694 272.095-272.093 29.703 29.704c8.188 8.186 8.188 21.516 0 29.703 \
       L203.602 439.404c-8.186 8.188-21.516 8.188-29.704 0z'%3E%3C/path%3E%3C/svg%3E";

/// # SOption element
///
/// This is included in the list of options for a [crate::component::SearchableSelect]
#[derive(PartialEq, Clone)]
pub struct SOption {
    /// value of the option as AttrValue
    pub value: AttrValue,

    /// Title of the option, ie what is displayed
    pub title: AttrValue,

    /// Item is selected
    pub selected: bool,

    /// Item is a header. Value is not used in this case.
    pub header: bool,

    /// Item is disabled
    pub disabled: bool,

    // Optional additional classes
    pub classes: Classes,
}

impl Default for SOption {
    fn default() -> Self {
        SOption {
            value: AttrValue::from(""),
            title: AttrValue::from(""),
            selected: false,
            header: false,
            disabled: false,
            classes: Classes::default()
        }
    }
}

/// # Properties for [SOptionComp] component.
///
/// This should not be used directly, provide vector of [SOption] instead
/// to [crate::SearchableSelect]
#[derive(Properties, PartialEq)]
pub (crate) struct SOptionProps {
    /// Properties derived from SOption
    pub attrs: SOption,

    /// Element is active
    #[prop_or_default]
    pub active: bool,

    /// Node reference to the <li> element to make it scroll
    pub node_ref: Option<NodeRef>,

    /// Called when the user clicks on the item with new value as parameter.
    pub onselectchange: Callback<(AttrValue, bool)>,
}

/// # Option for [crate::SearchableSelect]
///
/// Display one option inside the select block.
#[function_component]
pub (crate) fn SOptionComp(props: &SOptionProps) -> Html {
    let disabled = props.attrs.disabled && !props.attrs.header;
    let active = props.active && !props.attrs.header;

    let class = classes!(
        "list-group-item",
        (!props.attrs.header).then_some("list-group-item-action"),
        active.then_some("active"),
        disabled.then_some("disabled"),
        props.attrs.selected.then_some("selected"),
        props.attrs.header.then_some("header"),
        props.attrs.classes.clone()
    );

    let onclick = {
        let value = props.attrs.value.clone();
        let selected = props.attrs.selected;
        let onselectchange = props.onselectchange.clone();
        Callback::from(move |_| {
            let value = value.clone();
            onselectchange.emit((value, !selected));
        })
    };

    let selected = if props.attrs.selected {
        //format!(";background: url(\"{}\") no-repeat 5px center;background-size: 1em 1em;", CHECKMARK)
        format!(";background-image: url(\"{}\");background-repeat: no-repeat;
                  background-position: 5px center;background-size: 1em 1em;", CHECKMARK)
    } else {
        "".to_string()
    };

    let style = format!(
        "padding-left: 25px{}", selected
    );

    if props.attrs.header {
        html! {
            <li {class}>
                <h6>{ props.attrs.title.clone() }</h6>
            </li>
        }
    } else {
        html! {
            if let Some(node_ref) = props.node_ref.clone() {
                <a {class} {onclick} ref={node_ref} style={style}>{ props.attrs.title.clone() }</a>
            } else {
                <a {class} {onclick} style={style}>{ props.attrs.title.clone() }</a>
            }
        }
    }
}

