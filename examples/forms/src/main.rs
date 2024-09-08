use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
use yew::prelude::*;
use gloo_console::debug;

use yew_bootstrap::component::*;
use yew_bootstrap::icons::*;
use yew_bootstrap::util::*;
use yew_bootstrap::component::form::*;

#[derive(Debug)]
enum Msg {
    None,
    InputStr { name: String, value: String },
    InputStrChanged { name: String, value: String },
    InputBoolChanged { name: String, value: bool },
}

struct Model {
    last_input: String,
    input_changes: Vec<String>,
    value_text: AttrValue,
    value_textarea: AttrValue,
    value_range: AttrValue,
    value_datetime: AttrValue,
    value_select: AttrValue,
    value_color: AttrValue,
    value_radio: AttrValue,
    value_checkbox: bool,
    number_value: AttrValue,
    number_feedback: FormControlValidation,
    show_checkbox_tooltip: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            last_input: String::from(""),
            input_changes: vec!(),
            value_text: AttrValue::from(""),
            value_textarea: AttrValue::from(""),
            value_range: AttrValue::from("5"),
            value_datetime: AttrValue::from(""),
            value_select: AttrValue::from(""),
            value_color: AttrValue::from(""),
            value_radio: AttrValue::from("value-1"),
            value_checkbox: false,
            number_value: AttrValue::from(""),
            number_feedback: FormControlValidation::None,
            show_checkbox_tooltip: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        debug!(format!("Message received {msg:?}"));
        match msg {
            Msg::InputStr { name, value } => {
                self.last_input = format!("{name} changed to '{value}'");
                match &name[..] {
                    "input-text-callback" => {
                        self.value_text = AttrValue::from(value)
                    },
                    "input-number-validation" => {
                        self.number_value = AttrValue::from(value.clone());
                        self.number_feedback = FormControlValidation::None;
                        match value.parse::<i32>() {
                            Ok(n @ 10..=20) => {
                                self.number_feedback = FormControlValidation::Valid(Some(
                                    AttrValue::from(format!("Value {n} in correct range 10 to 20"))
                                ));
                            },
                            Ok(n) => {
                                self.number_feedback = FormControlValidation::Invalid(
                                    AttrValue::from(format!("Value {n} outside correct range 10 to 20"))
                                );
                            },
                            Err(_) => {
                                self.number_feedback = FormControlValidation::Invalid(
                                    AttrValue::from("Value cannot be parsed as a valid integer".to_string()))
                            }
                        }
                    },
                    _ => (),
                };
                true
            },
            Msg::InputStrChanged { name, value } => {
                self.input_changes.push(format!("{name} changed to {value}"));
                match &name[..] {
                    "input-textarea-callback" => self.value_textarea = AttrValue::from(value),
                    "input-range-callback" => self.value_range = AttrValue::from(value),
                    "input-datetime-callback" => self.value_datetime = AttrValue::from(value),
                    "input-color-callback" => self.value_color = AttrValue::from(value),
                    "input-select-callback" => self.value_select = AttrValue::from(value),
                    "input-radio-callback" => self.value_radio = AttrValue::from(value),
                    _ => {}
                };

                true
            },
            Msg::InputBoolChanged { name, value } => {
                self.input_changes.push(format!("{name} changed: {}", if value { "checked " } else { "unchecked" }));
                match &name[..] {
                    "input-checkbox-callback" => {
                        self.value_checkbox = value;
                    }
                    "input-tooltip-checkbox" => {
                        self.show_checkbox_tooltip = !value;
                    }
                    _ => {}
                }
                
                true
            },
            _ => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(move |event: InputEvent| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            debug!("oninput for ", input.clone());

            if let Some(input) = input {
                Msg::InputStr { name: input.name(), value: input.value() }
            } else {
                Msg::None
            }
        });

        let onchange = ctx.link().callback(move |event: Event| {
            let target: Option<EventTarget> = event.target();

            // Input element
            let input = target.clone().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                debug!("onchange for HtmlInputElement ", input.clone());
                let value = match &input.type_()[..] {
                    "checkbox" => Msg::InputBoolChanged { name: input.name(), value: input.checked() },
                    _ => Msg::InputStrChanged { name: input.name(), value: input.value() }
                };
                return value;
            }

            // Select element
            let input = target.clone().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                debug!("onchange for HtmlSelectElement ", input.clone());
                return Msg::InputStrChanged { name: input.name(), value: input.value() }
            }

            // TextArea element
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                debug!("onchange for HtmlAreaElement ", input.clone());
                return Msg::InputStrChanged { name: input.name(), value: input.value() }
            }

            Msg::None
        });

        let brand = BrandType::BrandIcon {
            text: AttrValue::from("Yew Bootstrap"),
            url: Some(AttrValue::from("https://yew.rs")),
            icon: BI::ROCKET,
        };

        let tooltip_select_ref = NodeRef::default();
        let tooltip_checkbox_ref = NodeRef::default();
        let tooltip_textarea_ref = NodeRef::default();

        html! {
            <>
                {include_inline()}
                {BIFiles::cdn()}
                <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-light bg-light" brand={brand}>
                    <NavItem text="link 1" />
                    <NavItem text="link 2" />
                    <NavDropdown text="several items">
                        <NavDropdownItem text="hello 1" />
                        <NavDropdownItem text="hello 2" />
                    </NavDropdown>
                </NavBar>
                <div id="forms" class="p-3">
                    <h1>{ "Forms" }</h1>
                    <h2>{ "Input types" }</h2>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl id="input-text" ctype={FormControlType::Text}
                            class="mb-3" label="Text" value="Initial text"/>
                        <FormControl id="input-textarea"
                            ctype={ FormControlType::TextArea { cols: None, rows: Some(5) } }
                            class="mb-3" label="Text area, 5 rows"  value="Some text"/>
                        <FormControl
                            id="input-email"
                            ctype={ FormControlType::Email { pattern: None } }
                            class="mb-3" 
                            label="Email"
                            autocomplete={ FormAutocompleteType::Email }
                        />
                        <FormControl 
                            id="input-password" 
                            ctype={ FormControlType::Password } 
                            class="mb-3" 
                            label="Password"
                            autocomplete={ FormAutocompleteType::Password { value: AutocompletePasswordType::NewPassword }}
                        />
                        <FormControl
                            id="input-url"
                            ctype={ FormControlType::Url { pattern: Some(AttrValue::from("https://.*")) } }
                            class="mb-3" 
                            label="Url with pattern"
                            autocomplete={ FormAutocompleteType::Url }
                        />
                        <FormControl
                            id="input-number"
                            ctype={ FormControlType::Number { min: Some(10), max: Some(20)} }
                            class="mb-3"
                            label="Number in range 10-20"
                            value="12"
                        />
                        <FormControl
                            id="input-range"
                            ctype={ FormControlType::Range { min: -10, max: 10, step: Some(5)} }
                            class="mb-3" label="Range -10-10, step 5" value="-5"
                        />
                        <FormControl
                            id="input-select1"
                            ctype={ FormControlType::Select}
                            class="mb-3"
                            label={ "Form control with options only "}
                        >
                            <SelectOption key=0 label="Select an option" selected=true />
                            <SelectOption key=1 label="Option 1" value="1"/>
                            <SelectOption key=2 label="Option 2" value="2"/>
                            <SelectOption key=3 label="Option 3" value="3"/>
                        </FormControl>
                        <FormControl
                            id="input-select2"
                            ctype={ FormControlType::Select}
                            class="mb-3"
                            label={ "Form control with optgroup and options "}
                        >
                            <SelectOption label="Select an option" selected=true />
                            <SelectOptgroup label="Group 1">
                                <SelectOption key=1 label="Option 1" value="1"/>
                                <SelectOption key=2 label="Option 2" value="2"/>
                            </SelectOptgroup>
                            <SelectOptgroup label="Group 2">
                                <SelectOption key=3 label="Option 3" value="3"/>
                                <SelectOption key=4 label="Option 4" value="4"/>
                            </SelectOptgroup>
                        </FormControl>
                        <FormControl
                            id="input-checkbox1"
                            ctype={ FormControlType::Checkbox }
                            class="mb-3"
                            label="Checkbox, unchecked"
                        />
                        <FormControl
                            id="input-checkbox2"
                            ctype={ FormControlType::Checkbox }
                            class="mb-3"
                            label="Checkbox, checked"
                            checked=true
                        />
                        <FormControl
                            id="input-radio1"
                            ctype={ FormControlType::Radio }
                            class="mb-3"
                            name="radio-name"
                            label="Radio, unchecked (Name 'radio-name')"
                        />
                        <FormControl
                            id="input-radio2"
                            ctype={ FormControlType::Radio }
                            class="mb-3"
                            name="radio-name"
                            checked=true
                            label="Radio, checked (Same name 'radio-name' to create a group)"
                        />
                        <FormControl
                            id="input-date1"
                            ctype={ FormControlType::Date }
                            class="mb-3"
                            label="Date (No range)"
                            value="2021-12-01"
                        />
                        <FormControl
                            id="input-date2"
                            ctype={
                                FormControlType::DateMinMax {
                                    min: Some(AttrValue::from("2023-01-01")),
                                    max: Some(AttrValue::from("2023-12-31"))
                                }
                            }
                            class="mb-3"
                            label="Date (2023 only)"
                        />
                        <FormControl
                            id="input-datetime1"
                            ctype={ FormControlType::Datetime }
                            class="mb-3"
                            label="Date and time (No range)"
                        />
                        <FormControl
                            id="input-datetime2"
                            ctype={
                                FormControlType::DatetimeMinMax {
                                    min: Some(AttrValue::from("2023-01-01T12:00")),
                                    max: Some(AttrValue::from("2023-01-01T18:00"))
                                }
                            }
                            class="mb-3"
                            label="Date and time (1st Jan 2023, 12:00 to 18:00)"
                            value="2023-01-01T15:00"
                        />
                        <FormControl
                            id="input-time1"
                            ctype={ FormControlType::Time }
                            class="mb-3"
                            label="Time (No range)"
                        />
                        <FormControl
                            id="input-time2"
                            ctype={
                                FormControlType::TimeMinMax {
                                    min: Some(AttrValue::from("12:00")),
                                    max: Some(AttrValue::from("18:00"))
                                }
                            }
                            class="mb-3"
                            label="Time (12:00 to 18:00)"
                        />
                        <FormControl
                            id="input-color"
                            ctype={ FormControlType::Color }
                            class="mb-3"
                            label="Color picker"
                        />
                        <FormControl
                            id="input-file"
                            ctype={
                                FormControlType::File {
                                    accept: vec!(AttrValue::from("image/png"), AttrValue::from(".pdf"))
                                }
                            }
                            class="mb-3"
                            label="Filename, accepts png images and .pdf files"
                        />
                        <FormControl id="input-hidden" ctype={ FormControlType::Hidden } class="mb-3" label="Hidden input" />
                    </Container>
                    <h2>{ "Help, placeholder, disabled" }</h2>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl
                            id="input-text-help"
                            ctype={FormControlType::Text}
                            class="mb-3" label="Text"
                            help={ Some(AttrValue::from("Some help text")) }
                            placeholder={ Some(AttrValue::from("Placeholder")) }
                        />
                        <FormControl
                            id="input-textarea-placeholder"
                            ctype={ FormControlType::TextArea { cols: None, rows: None } }
                            class="mb-3"
                            label="Text area with placeholder"
                            placeholder={ Some(AttrValue::from("Text as placeholder")) }
                        />
                        <FormControl
                            id="input-checkbox-help"
                            ctype={ FormControlType::Checkbox }
                            class="mb-3"
                            label="Checkbox with help"
                            help={ Some(AttrValue::from("Some help text")) }
                        />
                        <FormControl
                            id="input-time-disabled"
                            ctype={ FormControlType::Time }
                            class="mb-3"
                            label="Time disabled"
                            disabled=true
                            value="05:00"
                        />
                    </Container>
                    <h2>{ "Fields with tooltips"}</h2>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl
                            id="input-tooltip-text"
                            ctype={FormControlType::TextArea { cols: None, rows: None }}
                            class="mb-3" label="Text with tooltip on focus only"
                            placeholder="Placeholder text"
                            node_ref={tooltip_textarea_ref.clone()}
                        />
                        <Tooltip
                            target={tooltip_textarea_ref}
                            trigger_on_focus=true
                            trigger_on_hover=false
                        >
                            {"Tooltip for input control, shown when focussed."}
                        </Tooltip>
                        <FormControl
                            id="input-tooltip-select1"
                            ctype={FormControlType::Select}
                            class="mb-3"
                            label="Select control with tooltip on hover or focus"
                            node_ref={tooltip_select_ref.clone()}
                        >
                            <SelectOption key=0 label="Select an option" selected=true />
                            <SelectOption key=1 label="Option 1" value="1"/>
                            <SelectOption key=2 label="Option 2" value="2"/>
                            <SelectOption key=3 label="Option 3" value="3"/>
                        </FormControl>
                        <Tooltip
                            target={tooltip_select_ref}
                            placement={Placement::Bottom}
                        >
                            {"Tooltip for select control, shown when focussed or hovered."}
                        </Tooltip>
                        <FormControl
                            id="input-tooltip-checkbox"
                            name="input-tooltip-checkbox"
                            ctype={FormControlType::Checkbox}
                            class="mb-3"
                            label="I accept the terms and conditions to be able to hide the tooltip."
                            node_ref={tooltip_checkbox_ref.clone()}
                            onchange={onchange.clone()}
                            checked={!self.show_checkbox_tooltip}
                        />
                        <Tooltip
                            target={tooltip_checkbox_ref}
                            trigger_on_focus=false
                            trigger_on_hover=false
                            show={self.show_checkbox_tooltip}
                            placement={Placement::BottomStart}
                            fade={true}
                        >
                            {"You must accept the terms and conditions to hide this tooltip. Even though this "}
                            {"tooltip visually blocks other form elements, they can still receive events."}
                        </Tooltip>
                        {
                            for [
                                TooltipFocusTrigger::IfNoHover,
                                TooltipFocusTrigger::IfNoAnyHover,
                                TooltipFocusTrigger::Never,
                            ].iter().enumerate().map(|(i, trigger_on_focus)| {
                                let input_ref = NodeRef::default();

                                html_nested! {
                                    <>
                                        <FormControl
                                            id={format!("input-focus-trigger{i}")}
                                            ctype={FormControlType::Text}
                                            class="mb-3"
                                            label={format!("Input with tooltip on hover and on focus {trigger_on_focus:?}")}
                                            placeholder="Placeholder text"
                                            node_ref={input_ref.clone()}
                                        />
                                        <Tooltip
                                            target={input_ref}
                                            trigger_on_focus={*trigger_on_focus}
                                            trigger_on_hover=true
                                        >
                                            {format!("Tooltip for input with {trigger_on_focus:?}.")}
                                        </Tooltip>
                                    </>
                                }
                            })
                        }
                    </Container>
                    <h2>{ "Floating fields " }</h2>
                    <Container size={ContainerSize::ExtraLarge}>
                        <p>{
                            "Important: with floating set, label is required and placeholder is ignored. Not all field types are compatible."
                        }</p>
                        <FormControl
                            id="input-text-floating"
                            ctype={FormControlType::Text}
                            class="mb-3"
                            label="Text"
                            value="Initial text"
                            floating=true
                        />
                        <FormControl
                            id="input-textarea-floating"
                            ctype={ FormControlType::TextArea { cols: None, rows: Some(5) } }
                            class="mb-3"
                            label="Text area, 5 rows"
                            value="Some text"
                            floating=true
                        />
                        <FormControl
                            id="input-email-floating"
                            ctype={ FormControlType::Email { pattern: None } }
                            class="mb-3"
                            label="Email"
                            floating=true
                        />
                        <FormControl
                            id="input-password-floating"
                            ctype={ FormControlType::Password }
                            class="mb-3"
                            label="Password"
                            floating=true
                        />
                        <FormControl
                            id="input-url-floating"
                            ctype={ FormControlType::Url { pattern: Some(AttrValue::from("https://.*")) } }
                            class="mb-3"
                            label="Url with pattern"
                            floating=true
                        />
                        <FormControl
                            id="input-number"
                            ctype={ FormControlType::Number { min: Some(10), max: Some(20)} }
                            class="mb-3"
                            label="Number in range 10-20"
                            value="12"
                            floating=true
                        />
                        <FormControl
                            id="input-date1-floating"
                            ctype={ FormControlType::Date }
                            class="mb-3"
                            label="Date (No range)"
                            value="2021-12-01"
                            floating=true
                        />
                        <FormControl
                            id="input-time1-floating"
                            ctype={ FormControlType::Time }
                            class="mb-3"
                            label="Time (No range)"
                            floating=true
                        />
                        <FormControl
                            id="input-select-floating"
                            ctype={ FormControlType::Select}
                            class="mb-3"
                            label={ "Form control with floating label"}
                            floating=true
                        >
                            <SelectOption key=0 label="Select an option" selected=true />
                            <SelectOption key=1 label="Option 1" value="1"/>
                            <SelectOption key=2 label="Option 2" value="2"/>
                            <SelectOption key=3 label="Option 3" value="3"/>
                        </FormControl>
                    </Container>
                    <h2>{ "Form validation" }</h2>
                    <p>{
                        "Set feedback message to report a valid or invalid field. This sets the is-valid or is-invalid class"
                    }</p>
                    <p>{
                        "Note: look at the console in Developper mode to see the callbacks and messages."
                    }</p>
                    <Container size={ContainerSize::ExtraLarge}>
                        <p>{ "Change the value of the following control. Validation is computed on the provided input
                              when typing ('oninput' instead of 'onchange'), verifies if the value is a valid i32
                              and in the correct range" }</p>
                        <FormControl
                            id="input-number-validation"
                            name="input-number-validation"
                            ctype={ FormControlType::Number { min: None, max: None} }
                            class="mb-3"
                            label="Number in range 10-20"
                            value={ self.number_value.clone() }
                            oninput={ oninput.clone() }
                            help={ "Enter a value between 10 and 20"}
                            validation={ self.number_feedback.clone() }
                        />
                    </Container>
                    <h2>{ "Form events"}</h2>
                    <p>{ "Several events can be used" }</p>
                    <ul>
                        <li>{ "oninput: for text inputs, called each time a character is typed "}</li>
                        <li>{ "onchange: when an input is changed, and for a text input only when leaving the input."}</li>
                    </ul>
                    <p>{ "Concerning the value "}</p>
                    <ul>
                        <li>{ "Use 'name' to make difference between the fields instead of 'id' to have radio buttons working correctly," }</li>
                        <li>{ "For a checkbox, used 'checked()' to know if it is ticked or not,"}</li>
                        <li>{
                            "For a radio, use 'value()', and ensure 'name' is the same for the group, 'value' is set for each element in the group,
                            and 'checked' is set for the default selected element,"
                        }</li>
                        <li>{ "For other inputs, use 'value()'"}</li>
                    </ul>
                    <p>{
                        "Important: ensure value is sampled and passed to the FormControl.value or FormControl.checked, or value will be cleared
                        each time event is called. "
                    }</p>
                    <p>{
                        "Note: look at the console in Developper mode to see the callbacks and messages."
                    }</p>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl
                            id="input-text-callback"
                            name="input-text-callback"
                            ctype={FormControlType::Text}
                            class="mb-3"
                            label="Text with oninput and onchange"
                            value={ self.value_text.clone() }
                            onchange={ onchange.clone() }
                            oninput={ oninput.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_text))) }
                        />
                        <FormControl
                            id="input-textarea-callback"
                            name="input-textarea-callback"
                            ctype={FormControlType::TextArea { cols: None, rows: None }}
                            class="mb-3"
                            label="Textarea with oninput and onchange"
                            value={ self.value_textarea.clone() }
                            onchange={ onchange.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_textarea))) }
                        />
                        <FormControl
                            id="input-range-callback"
                            name="input-range-callback"
                            ctype={ FormControlType::Range { min: 0, max: 10, step: None } }
                            class="mb-3"
                            label="Range with onchange"
                            value={ self.value_range.clone() }
                            onchange={ onchange.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_range))) }
                        />
                        <FormControl
                            id="input-select-callback"
                            name="input-select-callback"
                            ctype={ FormControlType::Select} class="mb-3"
                            label={ "Form control with onchange"}
                            onchange={ onchange.clone() }
                            value={ self.value_select.clone()}
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_select))) }
                        >
                            <SelectOption key=0 label="Select an option" selected={ &self.value_select[..].is_empty() } />
                            <SelectOption key=1 label="Option 1" value="1" selected={ &self.value_select[..] == "1" }/>
                            <SelectOption key=2 label="Option 2" value="2" selected={ &self.value_select[..] == "2" }/>
                            <SelectOption key=3 label="Option 3" value="3" selected={ &self.value_select[..] == "3" }/>
                        </FormControl>
                        <FormControl
                            id="input-checkbox-callback"
                            name="input-checkbox-callback"
                            ctype={ FormControlType::Checkbox }
                            class="mb-3"
                            label="Checkbox with onchange"
                            checked={ self.value_checkbox }
                            onchange={ onchange.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_checkbox))) }
                        />
                        <FormControl
                            id="input-radio1-callback"
                            name="input-radio-callback"
                            ctype={ FormControlType::Radio }
                            class="mb-3"
                            label="Radio, value-1"
                            value="value-1"
                            onchange={ onchange.clone() }
                            checked={ &self.value_radio[..] == "value-1" }
                        />
                        <FormControl
                            id="input-radio2-callback"
                            name="input-radio-callback"
                            ctype={ FormControlType::Radio }
                            class="mb-3"
                            label="Radio, value-2"
                            value="value-2"
                            onchange={ onchange.clone() }
                            checked={ &self.value_radio[..] == "value-2" }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_radio))) }
                        />
                        <FormControl
                            id="input-datetime-callback"
                            name="input-datetime-callback"
                            ctype={ FormControlType::Datetime }
                            class="mb-3"
                            label="Date and time with onchange"
                            onchange={ onchange.clone() }
                            value={ self.value_datetime.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_datetime))) }
                        />
                        <FormControl
                            id="input-color-callback"
                            name="input-color-callback"
                            ctype={ FormControlType::Color }
                            class="mb-3"
                            label="Color picker with onchange"
                            onchange={ onchange.clone() }
                            value={ self.value_color.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_color))) }
                        />
                        <p>{ "List of changes below:" }</p>
                        {
                            self.input_changes.iter().clone().map(move |change| {
                                html! { <p>{ change }</p> }
                            }).collect::<Html>()
                        }
                    </Container>
                </div>
                { include_cdn_js() }
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}