use yew::prelude::*;
use super::*;

/// Validation type for a form control, with feedback message
#[derive(Clone, PartialEq)]
pub enum FormControlValidation {
    /// Form field has not been validated or nothing to show
    None,
    /// Valid validation with optional feedback message
    Valid(Option<AttrValue>),
    /// Invalid validation with feedback message
    Invalid(AttrValue),
}


/// # Properties for a FormControl
#[derive(Properties, Clone, PartialEq)]
pub struct FormControlProps {
    /// Type of control
    pub ctype: FormControlType,

    /// Id for the form field
    pub id: AttrValue,

    /// CSS class
    #[prop_or_default]
    pub class: Classes,

    /// Optional label for the control
    #[prop_or_default]
    pub label: Option<AttrValue>,

    /// Optional placeholder, only used for text fields
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Optional help text
    #[prop_or_default]
    pub help: Option<AttrValue>,

    /// Name for the form field.
    /// For [FormControlType::Radio], set same name to create a group
    #[prop_or_default]
    pub name: AttrValue,

    /// Value as string, ignored for checkbox (Use `checked` instead). For a radio,
    /// indicates the value in the group
    #[prop_or_default]
    pub value: AttrValue,

    /// Is this field required? Defaults to false.
    #[prop_or_default]
    pub required: bool,

    /// Checked or default value:
    ///
    /// - For a checkbox, indicates the state (Checked or not)
    /// - For a radio, indicates the default value (Only one in the group should have it)
    #[prop_or_default]
    pub checked: bool,

    /// Disabled if true
    #[prop_or_default]
    pub disabled: bool,

    /// If true, label is floating inside the input. Ignored for checkbox/radio, date/time,
    /// color, range fields.
    ///
    /// When true, `label` is required and `placeholder` is ignored.
    #[prop_or_default]
    pub floating: bool,

    /// Multiple select, only used for select form input
    #[prop_or_default]
    pub multiple: bool,

    /// Children, only used for select form input
    #[prop_or_default]
    pub children: Children,

    /// Form validation feedback
    /// Note: you must always validate user input server-side as well, this is only provided for better user experience
    #[prop_or(FormControlValidation::None)]
    pub validation: FormControlValidation,

    /// Optional onchange event applied on the input
    /// For a text input, this is called when leaving the input field
    #[prop_or_default]
    pub onchange: Callback<Event>,

    /// Optional oninput event applied on the input, only for text inputs
    /// This is called each time an input is received, after each character
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,

    /// Optional onclick event applied on the input
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}


/// Convert an option (Typically integer) to an AttrValue option
fn convert_to_string_option<T>(value: &Option<T>) -> Option<AttrValue>
where T: std::fmt::Display {
    value.as_ref().map(|v| AttrValue::from(v.to_string()))
}

/// # Form Control field
///
/// Global form control for most Bootstrap form fields. See:
///
/// - [FormControlType] to define the type of input
/// - [FormControlProps] to list the properties for the component. Note that
///   some fields are ignored or may not work correctly depending on the type
///   of input (See Bootstrap documentation for details)
///
/// ## Examples
///
/// Basic text field:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::form::*;
/// fn test() -> Html {
///   html! {
///     <FormControl
///         id="input-text"
///         ctype={FormControlType::Text}
///         class="mb-3"
///         label="Text"
///         value="Initial text"
///     />
///   }
/// }
/// ```
///
/// Some input types need parameters for the `ctype` enum. Optional parameters use `Option` enums.
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::form::*;
/// fn test() -> Html {
///   html! {
///     <FormControl
///         id="input-number"
///         ctype={
///             FormControlType::Number {
///                 min: Some(10),
///                 max: Some(20)
///             }
///         }
///         class="mb-3"
///         label="Number in range 10-20"
///         value="12"
///     />
///   }
/// }
/// ```
///
/// Almost all properties are `AttrValue` type, and need to be converted into the
/// correct format, as required by the input. For example for a DateTime with range
/// input:
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::form::*;
/// fn test() -> Html {
///   html! {
///     <FormControl
///         id="input-datetime2"
///         ctype={
///             FormControlType::DatetimeMinMax {
///                 min: Some(AttrValue::from("2023-01-01T12:00")),
///                 max: Some(AttrValue::from("2023-01-01T18:00"))
///             }
///         }
///         class="mb-3"
///         label="Date and time (1st Jan 2023, 12:00 to 18:00)"
///         value="2023-01-01T15:00"
///     />
///   }
/// }
/// ```
///
/// Select input is the only input that can receive children, of type [SelectOption]
/// or [SelectOptgroup]. For example:
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::form::*;
/// fn test() -> Html {
///   html! {
///     <FormControl
///         id="input-select-feedback"
///         ctype={ FormControlType::Select}
///         class="mb-3"
///         label={ "Form control invalid" }
///         validation={
///             FormControlValidation::Invalid(AttrValue::from("Select an option"))
///         }
///     >
///       <SelectOption key=0 label="Select an option" selected=true />
///       <SelectOption key=1 label="Option 1" value="1"/>
///       <SelectOption key=2 label="Option 2" value="2"/>
///       <SelectOption key=3 label="Option 3" value="3"/>
///     </FormControl>
///   }
/// }
/// ```
///
/// `onclick`, `oninput` and `onchange` events are available. `onchange` should be preferred
/// for most inputs, but for text inputs (`Text`, `TextArea`, `Number`, etc), `onchange` is
/// only called when the input looses focus, while `oninput` is called each time a key is
/// pressed.
///
/// In the callback, the target needs to be converted to a descendent of `HtmlElement`
/// to access the fields (Like `type`, `name` and `value`). All inputs can be converted
/// to `HtmlInputElement` but `Select` and `TextArea`. This is an example of callback
/// function to convert to the correct type; `checkbox` is special as the `checked`
/// property should be used.
///
/// Note: `HtmlTextAreaElement` and `HtmlSelectElement` are not enabled by default
/// and need the feature to be required:
///
/// ```toml
/// web-sys = { version = "0.3.*", features = ["HtmlTextAreaElement", "HtmlSelectElement"] }
/// ```
///
/// ```rust
/// use wasm_bindgen::JsCast;
/// use web_sys::{EventTarget, HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
/// use yew::prelude::*;
///
/// enum Msg {
///   None,
///   InputStrChanged { name: String, value: String },
///   InputBoolChanged { name: String, value: bool },
/// }
///
/// let onchange = Callback::from(move |event: Event| -> Msg {
///   let target: Option<EventTarget> = event.target();
///
///   // Input element
///   let input = target.clone().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
///   if let Some(input) = input {
///       let value = match &input.type_()[..] {
///           "checkbox" => Msg::InputBoolChanged { name: input.name(), value: input.checked() },
///           _ => Msg::InputStrChanged { name: input.name(), value: input.value() }
///       };
///       return value;
///   }
///
///   // Select element
///   let input = target.clone().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
///   if let Some(input) = input {
///       return Msg::InputStrChanged { name: input.name(), value: input.value() }
///   }
///
///   // TextArea element
///   let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
///   if let Some(input) = input {
///       return Msg::InputStrChanged { name: input.name(), value: input.value() }
///   }
///
///   Msg::None
/// });
/// ```

#[function_component]
pub fn FormControl(props: &FormControlProps) -> Html {
    let label = match props.label.clone() {
        None => None,
        Some(text) => {
            let class = if props.floating { None } else { Some("form-label") };
            Some(html! {
                <label for={ props.id.clone() } class={ class }>{ text.clone() }</label>
            })
        }
    };

    let help = props.help.as_ref().map(|text| html! {
        <div class="form-text">{ text.clone() }</div>
    });

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

    let pattern = match &props.ctype {
        FormControlType::Email{ pattern } => pattern,
        FormControlType::Url{ pattern } => pattern,
        _ => &None,
    };

    // Placeholder required when `floating` is set, assign to label
    let mut placeholder = props.placeholder.clone();
    if props.floating && placeholder.is_none() {
        placeholder = Some(props.label.clone().expect("When floating is set, label cannot be None"));
    }

    match &props.ctype {
        FormControlType::TextArea { cols, rows } => {
            let mut classes = classes!(props.class.clone());
            if props.floating {
                classes.push("form-floating");
            }

            let input_classes = classes!("form-control", validation_class);

            let cols_str = convert_to_string_option(cols);
            let rows_str = convert_to_string_option(rows);
            let (label_before, label_after) =
                if props.floating { (None, label) } else { (label, None) };

            html! {
                <div class={ classes }>
                    { label_before }
                    <textarea
                        class={ input_classes }
                        id={ props.id.clone() }
                        name={ props.name.clone() }
                        cols={ cols_str }
                        rows={ rows_str }
                        placeholder={ placeholder }
                        value={ props.value.clone() }
                        disabled={ props.disabled }
                        oninput={props.oninput.clone() }
                        onchange={ props.onchange.clone() }
                        onclick={ props.onclick.clone() }
                        required={ props.required }
                    />
                    { label_after }
                    { help }
                    { validation }
                </div>
            }
        },
        FormControlType::Select => {
            let mut classes = classes!(props.class.clone());
            if props.floating {
                classes.push("form-floating");
            }

            let input_classes = classes!("form-select", validation_class);

            let (label_before, label_after) =
                if props.floating { (None, label) } else { (label, None) };

            html! {
                <div class={ classes }>
                    { label_before }
                    <select
                        class={ input_classes }
                        id={ props.id.clone()}
                        name={ props.name.clone() }
                        disabled={ props.disabled }
                        onchange={ props.onchange.clone() }
                        onclick={ props.onclick.clone() }
                        required={ props.required }
                    >
                        { for props.children.clone() }
                    </select>
                    { label_after }
                    { help }
                    { validation }
                </div>
            }
        },
        FormControlType::Checkbox | FormControlType::Radio => {
            let mut classes = classes!("form-check");
            classes.push(props.class.clone());

            let input_classes = classes!("form-check-input", validation_class);

            html! {
                <div class={ classes }>
                    <input
                        type={ props.ctype.to_str() }
                        class={ input_classes }
                        id={ props.id.clone() }
                        name={ props.name.clone() }
                        checked={ props.checked }
                        disabled={ props.disabled }
                        value={ props.value.clone() }
                        onchange={ props.onchange.clone() }
                        onclick={ props.onclick.clone() }
                        required={ props.required }
                    />
                    { label }
                    { help }
                    { validation}
                </div>
            }
        },
        _ => {
            let mut min_str = None;
            let mut max_str = None;
            let mut step_str = None;
            let mut accept_str = None;
            match &props.ctype {
                FormControlType::Number { min, max } => {
                    min_str = convert_to_string_option(min);
                    max_str = convert_to_string_option(max);
                },
                FormControlType::Range { min, max, step } => {
                    min_str = Some(AttrValue::from(min.to_string()));
                    max_str = Some(AttrValue::from(max.to_string()));
                    step_str = convert_to_string_option(step);
                },
                FormControlType::DateMinMax { min, max } |
                FormControlType::DatetimeMinMax { min, max } |
                FormControlType::TimeMinMax { min, max } => {
                    min_str = min.clone();
                    max_str = max.clone();
                },
                FormControlType::File { accept } => {
                    let accept_vec : Vec<String> = accept.clone().iter().cloned().map(
                        move |value| { value.to_string() }
                    ).collect();
                    accept_str = Some(accept_vec.join(", "));
                }
                _ => ()
            }

            let mut classes = classes!(props.class.clone());
            if props.floating {
                classes.push("form-floating");
            }

            let input_classes = classes!("form-control", validation_class);

            let (label_before, label_after) =
                if props.floating { (None, label) } else { (label, None) };

            html! {
                <div class={ classes }>
                    { label_before }
                    <input
                        type={ props.ctype.to_str() }
                        class={ input_classes }
                        id={ props.id.clone() }
                        name={ props.name.clone() }
                        value={ props.value.clone() }
                        pattern={ pattern }
                        accept={ accept_str }
                        placeholder={ placeholder }
                        min={ min_str }
                        max={ max_str }
                        step={ step_str }
                        disabled={ props.disabled }
                        onchange={ props.onchange.clone() }
                        onclick={ props.onclick.clone() }
                        oninput={ props.oninput.clone() }
                        required={ props.required }
                    />
                    { label_after }
                    { help }
                    { validation }
                </div>
            }
        }
    }
}