use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
use yew::prelude::*;

use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
use yew_bootstrap::form::*;
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputStr { name, value } => {
                self.last_input = format!("{name} changed to '{value}'");
                if &name[..] == "input-text-callback" {
                    self.value_text = AttrValue::from(value)
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
                if &name[..] == "input-checkbox-callback" {
                    self.value_checkbox = value;
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
                let value = match &input.type_()[..] {
                    "checkbox" => Msg::InputBoolChanged { name: input.name(), value: input.checked() },
                    _ => Msg::InputStrChanged { name: input.name(), value: input.value() }
                };
                return value;
            }

            // Select element
            let input = target.clone().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                return Msg::InputStrChanged { name: input.name(), value: input.value() }
            }

            // TextArea element
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                return Msg::InputStrChanged { name: input.name(), value: input.value() }
            }

            Msg::None
        });

        let brand = BrandType::BrandIcon {
            text: AttrValue::from("Yew Bootstrap"),
            url: Some(AttrValue::from("https://yew.rs")),
            icon: AttrValue::from("rocket")
        };

        html! {
            <>
                {include_inline()}
                {include_cdn_icons()}
                <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-light bg-light" brand={brand}>
                    <NavItem text="link 1" />
                    <NavItem text="link 2" />
                    <NavDropdown text="several items">
                        <NavDropdownItem text="hello 1" />
                        <NavDropdownItem text="hello 2" />
                    </NavDropdown>
                </NavBar>
                <Modal id="ExampleModal">
                    <ModalHeader title="Modal title" id="ExampleModal" />
                    <ModalBody>
                        <p>{"Modal body text goes here."}</p>
                    </ModalBody>
                    <ModalFooter>
                        <Button class="btn-secondary" modal_dismiss={true}>{"Close"}</Button>
                        <Button class="btn-primary">{"Save changes"}</Button>
                    </ModalFooter>
                </Modal>
                <div id="layout" class="p-3">
                    <h1>{ "Containers" }</h1>
                    <Container class="bg-primary">{"Normal"}</Container>
                    <Container class="bg-secondary" fluid={true}>{"Fluid"}</Container>
                    <Container class="bg-success" size={ContainerSize::Small}>{"Small"}</Container>
                    <Container class="bg-danger" size={ContainerSize::Medium}>{"Medium"}</Container>
                    <Container class="bg-warning" size={ContainerSize::Large}>{"Large"}</Container>
                    <Container class="bg-info" size={ContainerSize::ExtraLarge}>{"Extra Large"}</Container>
                    <Container class="bg-light" size={ContainerSize::ExtraExtraLarge}>{"Extra Large"}</Container>

                    <h1>{ "Grid" }</h1>
                    <Row>
                        <Column class="bg-info">
                            {"1 of 2"}
                        </Column>
                        <Column class="bg-primary">
                            {"2 of 2"}
                        </Column>
                    </Row>
                    <Row>
                        <Column class="bg-danger">
                            {"1 of 3"}
                        </Column>
                        <Column class="bg-warning">
                            {"2 of 3"}
                        </Column>
                        <Column class="bg-success">
                            {"3 of 3"}
                        </Column>
                    </Row>
                    <Row>
                        <Column class="bg-info">
                            {"1 of 3"}
                        </Column>
                        <Column size={6} class="bg-secondary">
                            {"2 of 3 (wider)"}
                        </Column>
                        <Column class="bg-primary">
                            {"3 of 3"}
                        </Column>
                    </Row>
                    <Row>
                        <Column class="bg-danger">
                            {"1 of 3"}
                        </Column>
                        <Column size={5} class="bg-secondary">
                            {"2 of 3 (wider)"}
                        </Column>
                        <Column class="bg-success">
                            {"3 of 3"}
                        </Column>
                    </Row>
                    <Row>
                        <Column size={None} md={5} class="bg-info">
                            {"md-only"}
                        </Column>
                    </Row>
                </div>
                <div id="components" class="p-3">
                    <h1>{ "Alerts" }</h1>
                    <Alert style={Color::Primary}>
                        { "This is a primary alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Secondary}>
                        { "This is a secondary alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Success}>
                        { "This is a success alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Danger}>
                        { "This is a danger alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Warning}>
                        { "This is a warning alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Info}>
                        { "This is a info alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Light}>
                        { "This is a light alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Dark}>
                        { "This is a dark alert—check it out!" }
                    </Alert>
                    <Alert style={Color::Link}>
                        { "This is a link alert—check it out!" }
                    </Alert>
                    <h1>{"Modals"}</h1>
                    <Button style={Color::Primary} modal_target={"ExampleModal"}>{"Open Modal"}</Button>

                    <h1>{"Buttons"}</h1>
                    <Button style={Color::Primary}>{"Primary"}</Button>
                    <Button style={Color::Secondary}>{"Secondary"}</Button>
                    <Button style={Color::Success}>{"Success"}</Button>
                    <Button style={Color::Danger}>{"Danger"}</Button>
                    <Button style={Color::Warning}>{"Warning"}</Button>
                    <Button style={Color::Info}>{"Info"}</Button>
                    <Button style={Color::Light}>{"Light"}</Button>
                    <Button style={Color::Dark}>{"Dark"}</Button>
                    <Button style={Color::Link}>{"Link"}</Button>

                    <h2>{"Outline buttons"}</h2>
                    <Button style={Color::Primary} outline={true}>{"Primary"}</Button>
                    <Button style={Color::Secondary} outline={true}>{"Secondary"}</Button>
                    <Button style={Color::Success} outline={true}>{"Success"}</Button>
                    <Button style={Color::Danger} outline={true}>{"Danger"}</Button>
                    <Button style={Color::Warning} outline={true}>{"Warning"}</Button>
                    <Button style={Color::Info} outline={true}>{"Info"}</Button>
                    <Button style={Color::Light} outline={true}>{"Light"}</Button>
                    <Button style={Color::Dark} outline={true}>{"Dark"}</Button>
                    <Button style={Color::Link} outline={true} text="Link2" />

                    <h2>{"Sizes"}</h2>
                    <Button style={Color::Primary} size={ButtonSize::Large}>{"Large button"}</Button>
                    <Button style={Color::Secondary} size={ButtonSize::Large}>{"Large button"}</Button>
                    <br />
                    <Button style={Color::Primary} size={ButtonSize::Normal}>{"Normal button"}</Button>
                    <Button style={Color::Secondary} size={ButtonSize::Normal}>{"Normal button"}</Button>
                    <br />
                    <Button style={Color::Primary} size={ButtonSize::Small}>{"Small button"}</Button>
                    <Button style={Color::Secondary} size={ButtonSize::Small}>{"Small button"}</Button>

                    <h2>{"Disabled state"}</h2>
                    <Button style={Color::Primary} disabled={true}>{"Primary"}</Button>
                    <Button style={Color::Secondary} disabled={true}>{"Secondary"}</Button>

                    <h2>{"Block buttons"}</h2>
                    <div class="d-grid gap-2">
                        <Button style={Color::Primary} block={true}>{"Primary"}</Button>
                        <Button style={Color::Secondary} block={true}>{"Secondary"}</Button>
                    </div>

                    <h1>{"Button groups"}</h1>
                    <ButtonGroup>
                        <Button style={Color::Primary}>{"Primary"}</Button>
                        <Button style={Color::Secondary}>{"Secondary"}</Button>
                    </ButtonGroup>

                    <h2>{"Vertical variation"}</h2>
                    <ButtonGroup vertical={true}>
                        <Button style={Color::Primary}>{"Primary"}</Button>
                        <Button style={Color::Secondary}>{"Secondary"}</Button>
                    </ButtonGroup>
                </div>
                <div id="helpers" class="p-3">
                    <h1>{"Vertical/Horizontal rule"}</h1>
                    <h2>{"Horizontal"}</h2>
                    <Line />
                    <Line style={Color::Primary} />
                    <Line height={Size::Px(5)} />
                    <Line width={Size::Px(100)} />
                    <h2>{"Vertical"}</h2>
                    <Line vertical={true} /><br />
                    <Line vertical={true} style={Color::Primary} /><br />
                    <Line vertical={true} height={Size::Px(50)} /><br />
                    <Line vertical={true} width={Size::Px(100)} /><br />
                </div>
                <div id="forms" class="p-3">
                    <h1>{ "Forms" }</h1>
                    <h2>{ "Input types" }</h2>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl id="input-text" ctype={FormControlType::Text}
                            class="mb-3" label="Text" value="Initial text"/>
                        <FormControl id="input-textarea"
                            ctype={ FormControlType::TextArea { cols: None, rows: Some(5) } }
                            class="mb-3" label="Text area, 5 rows"  value="Some text"/>
                        <FormControl id="input-email"
                                     ctype={ FormControlType::Email { pattern: None } }
                                     class="mb-3" label="Email"/>
                        <FormControl id="input-password" ctype={ FormControlType::Password }
                                     class="mb-3" label="Password"/>
                        <FormControl id="input-url"
                                     ctype={ FormControlType::Url { pattern: Some(AttrValue::from("https://.*")) } }
                                     class="mb-3" label="Url with pattern"/>
                        <FormControl id="input-number" ctype={ FormControlType::Number { min: Some(10), max: Some(20)} }
                                     class="mb-3" label="Number in range 10-20" value="12"/>
                        <FormControl id="input-range" ctype={ FormControlType::Range { min: -10, max: 10, step: Some(5)} }
                                     class="mb-3" label="Range -10-10, step 5" value="-5"/>
                        <FormControl id="input-select1" ctype={ FormControlType::Select} class="mb-3"
                                     label={ "Form control with options only "}>
                            <SelectOption key=0 label="Select an option" selected=true />
                            <SelectOption key=1 label="Option 1" value="1"/>
                            <SelectOption key=2 label="Option 2" value="2"/>
                            <SelectOption key=3 label="Option 3" value="3"/>
                        </FormControl>
                        <FormControl id="input-select2" ctype={ FormControlType::Select} class="mb-3"
                                     label={ "Form control with optgroup and options "}>
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
                        <FormControl id="input-checkbox1" ctype={ FormControlType::Checkbox }
                                     class="mb-3" label="Checkbox, unchecked"/>
                        <FormControl id="input-checkbox2" ctype={ FormControlType::Checkbox }
                                     class="mb-3" label="Checkbox, checked" checked=true/>
                        <FormControl id="input-radio1" ctype={ FormControlType::Radio }
                                     class="mb-3" name="radio-name"
                                     label="Radio, unchecked (Name 'radio-name')"/>
                        <FormControl id="input-radio2" ctype={ FormControlType::Radio }
                                     class="mb-3" name="radio-name" checked=true
                                     label="Radio, checked (Same name 'radio-name' to create a group)" />
                        <FormControl id="input-date1" ctype={ FormControlType::Date }
                                     class="mb-3" label="Date (No range)" value="2021-12-01"/>
                        <FormControl id="input-date2"
                                     ctype={ FormControlType::DateMinMax {
                                        min: Some(AttrValue::from("2023-01-01")),
                                        max: Some(AttrValue::from("2023-12-31")) } }
                                     class="mb-3" label="Date (2023 only)" />
                        <FormControl id="input-datetime1" ctype={ FormControlType::Datetime }
                                     class="mb-3" label="Date and time (No range)" />
                        <FormControl id="input-datetime2"
                                     ctype={ FormControlType::DatetimeMinMax {
                                        min: Some(AttrValue::from("2023-01-01T12:00")),
                                        max: Some(AttrValue::from("2023-01-01T18:00")) } }
                                     class="mb-3" label="Date and time (1st Jan 2023, 12:00 to 18:00)"
                                     value="2023-01-01T15:00" />
                        <FormControl id="input-time1" ctype={ FormControlType::Time }
                                     class="mb-3" label="Time (No range)" />
                        <FormControl id="input-time2"
                                     ctype={ FormControlType::TimeMinMax {
                                        min: Some(AttrValue::from("12:00")),
                                        max: Some(AttrValue::from("18:00")) } }
                                     class="mb-3" label="Time (12:00 to 18:00)" />
                        <FormControl id="input-color" ctype={ FormControlType::Color }
                                     class="mb-3" label="Color picker" />
                        <FormControl id="input-file" ctype={ FormControlType::File {
                                         accept: vec!(AttrValue::from("image/png"), AttrValue::from(".pdf")) } }
                                     class="mb-3" label="Filename, accepts png images and .pdf files" />
                        <FormControl id="input-hidden" ctype={ FormControlType::Hidden }
                                    class="mb-3" label="Hidden input" />
                   </Container>
                   <h2>{ "Help, placeholder, disabled" }</h2>
                   <Container size={ContainerSize::ExtraLarge}>
                        <FormControl id="input-text-help" ctype={FormControlType::Text}
                            class="mb-3" label="Text"
                            help={ Some(AttrValue::from("Some help text")) }
                            placeholder={ Some(AttrValue::from("Placeholder")) }/>
                        <FormControl id="input-textarea-placeholder"
                            ctype={ FormControlType::TextArea { cols: None, rows: None } }
                            class="mb-3" label="Text area with placeholder"
                            placeholder={ Some(AttrValue::from("Text as placeholder")) }/>
                        <FormControl id="input-checkbox-help" ctype={ FormControlType::Checkbox }
                            class="mb-3" label="Checkbox with help"
                            help={ Some(AttrValue::from("Some help text")) }/>
                        <FormControl id="input-time-disabled" ctype={ FormControlType::Time }
                            class="mb-3" label="Time disabled" disabled=true value="05:00"/>
                    </Container>
                    <h2>{ "Floating fields " }</h2>
                    <Container size={ContainerSize::ExtraLarge}>
                        <p>{ "Important: with floating set, label is required
                              and placeholder is ignored. Not all field types are compatible." }</p>
                        <FormControl id="input-text-floating" ctype={FormControlType::Text}
                            class="mb-3" label="Text" value="Initial text" floating=true />
                        <FormControl id="input-textarea-floating"
                            ctype={ FormControlType::TextArea { cols: None, rows: Some(5) } }
                            class="mb-3" label="Text area, 5 rows"  value="Some text" floating=true/>
                        <FormControl id="input-email-floating"
                            ctype={ FormControlType::Email { pattern: None } }
                            class="mb-3" label="Email" floating=true/>
                        <FormControl id="input-password-floating" ctype={ FormControlType::Password }
                            class="mb-3" label="Password" floating=true />
                        <FormControl id="input-url-floating"
                            ctype={ FormControlType::Url { pattern: Some(AttrValue::from("https://.*")) } }
                            class="mb-3" label="Url with pattern" floating=true/>
                        <FormControl id="input-number" ctype={ FormControlType::Number { min: Some(10), max: Some(20)} }
                                class="mb-3" label="Number in range 10-20" value="12" floating=true />
                        <FormControl id="input-date1-floating" ctype={ FormControlType::Date }
                                    class="mb-3" label="Date (No range)" value="2021-12-01" floating=true/>
                        <FormControl id="input-time1-floating" ctype={ FormControlType::Time }
                                    class="mb-3" label="Time (No range)" floating=true />
                        <FormControl id="input-select-floating" ctype={ FormControlType::Select} class="mb-3"
                                     label={ "Form control with floating label"} floating=true>
                           <SelectOption key=0 label="Select an option" selected=true />
                           <SelectOption key=1 label="Option 1" value="1"/>
                           <SelectOption key=2 label="Option 2" value="2"/>
                           <SelectOption key=3 label="Option 3" value="3"/>
                       </FormControl>
                   </Container>
                    <h2>{ "Form validation" }</h2>
                    <p>{ "Set feedback message to report a valid or invalid field. This sets the is-valid or
                         is-invalid class" }</p>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl id="input-text-valid" ctype={FormControlType::Text}
                            class="mb-3" label="Valid text, no message" value="Initial text"
                            validation={ FormControlValidation::Valid(None) } />
                        <FormControl id="input-textarea-valid"
                            ctype={ FormControlType::TextArea { cols: None, rows: Some(5) } }
                            class="mb-3" label="Valid text area, message"  value="Some text"
                            validation={ FormControlValidation::Valid(Some(AttrValue::from("Value is valid"))) }/>
                        <FormControl id="input-number" ctype={ FormControlType::Number { min: Some(10), max: Some(20)} }
                            class="mb-3" label="Number in range 10-20" value="8"
                            validation={ FormControlValidation::Invalid(AttrValue::from("Value needs to be in range 10-20")) }/>
                        <FormControl id="input-checkbox-valid" ctype={ FormControlType::Checkbox }
                            class="mb-3" label="Checkbox, valid" checked=true
                            validation={ FormControlValidation::Valid(None) }/>
                        <FormControl id="input-checkbox-invalid" ctype={ FormControlType::Checkbox }
                                class="mb-3" label="Checkbox, invalid"
                                validation={ FormControlValidation::Invalid(AttrValue::from("You must click this one")) }/>
                        <FormControl id="input-select-invalid" ctype={ FormControlType::Select} class="mb-3"
                                     label={ "Form control invalid"}
                                     validation={ FormControlValidation::Invalid(AttrValue::from("Select an option")) }>
                            <SelectOption key=0 label="Select an option" selected=true />
                            <SelectOption key=1 label="Option 1" value="1"/>
                            <SelectOption key=2 label="Option 2" value="2"/>
                            <SelectOption key=3 label="Option 3" value="3"/>
                        </FormControl>
                    </Container>
                    <h2>{ "Form events"}</h2>
                    <p>{ "Several events can be used" }</p>
                    <ul>
                        <li>{ "oninput: for text inputs, called each time a character is typed "}</li>
                        <li>{ "onchange: when an input is changed, and for a text input only when
                               leaving the input."}</li>
                    </ul>
                    <p>{ "Concerning the value "}</p>
                    <ul>
                        <li>{ "Use 'name' to make difference between the fields instead of 'id' to have
                               radio buttons working correctly," }</li>
                        <li>{ "For a checkbox, used 'checked()' to know if it is ticked or not,"}</li>
                        <li>{ "For a radio, use 'value()', and ensure 'name' is the same for the group,
                               'value' is set for each element in the group, and 'checked' is set for
                               the default selected element,"}</li>
                        <li>{ "For other inputs, use 'value()'"}</li>
                    </ul>
                    <p>{ "Important: ensure value is sampled and passed to the FormControl.value or
                          FormControl.checked, or value will be cleared each time event is called. "}</p>
                    <Container size={ContainerSize::ExtraLarge}>
                        <FormControl id="input-text-callback" name="input-text-callback"
                            ctype={FormControlType::Text}
                            class="mb-3" label="Text with oninput and onchange" value={ self.value_text.clone() }
                            onchange={ onchange.clone() } oninput={ oninput.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_text))) } />
                        <FormControl id="input-textarea-callback" name="input-textarea-callback"
                            ctype={FormControlType::TextArea { cols: None, rows: None }}
                            class="mb-3" label="Textarea with oninput and onchange" value={ self.value_textarea.clone() }
                            onchange={ onchange.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_textarea))) } />
                        <FormControl id="input-range-callback" name="input-range-callback"
                            ctype={ FormControlType::Range { min: 0, max: 10, step: None } }
                            class="mb-3" label="Range with onchange" value={ self.value_range.clone() }
                            onchange={ onchange.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_range))) } />
                        <FormControl id="input-select-callback" name="input-select-callback"
                            ctype={ FormControlType::Select} class="mb-3"
                            label={ "Form control with onchange"}
                            onchange={ onchange.clone() } value={ self.value_select.clone()}
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_select))) }>
                            <SelectOption key=0 label="Select an option" selected={ &self.value_select[..] == "" } />
                            <SelectOption key=1 label="Option 1" value="1" selected={ &self.value_select[..] == "1" }/>
                            <SelectOption key=2 label="Option 2" value="2" selected={ &self.value_select[..] == "2" }/>
                            <SelectOption key=3 label="Option 3" value="3" selected={ &self.value_select[..] == "3" }/>
                        </FormControl>
                        <FormControl id="input-checkbox-callback" name="input-checkbox-callback"
                            ctype={ FormControlType::Checkbox }
                            class="mb-3" label="Checkbox with onchange" checked={ self.value_checkbox }
                            onchange={ onchange.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_checkbox))) } />
                        <FormControl id="input-radio1-callback" name="input-radio-callback"
                            ctype={ FormControlType::Radio }
                            class="mb-3" label="Radio, value-1" value="value-1"
                            onchange={ onchange.clone() } checked={ &self.value_radio[..] == "value-1" }/>
                        <FormControl id="input-radio2-callback" name="input-radio-callback"
                            ctype={ FormControlType::Radio }
                            class="mb-3" label="Radio, value-2" value="value-2"
                            onchange={ onchange.clone() } checked={ &self.value_radio[..] == "value-2" }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_radio))) }/>
                        <FormControl id="input-datetime-callback" name="input-datetime-callback"
                            ctype={ FormControlType::Datetime }
                            class="mb-3" label="Date and time with onchange"
                            onchange={ onchange.clone() } value={ self.value_datetime.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_datetime))) }/>
                        <FormControl id="input-color-callback" name="input-color-callback"
                            ctype={ FormControlType::Color }
                            class="mb-3" label="Color picker with onchange"
                            onchange={ onchange.clone() } value={ self.value_color.clone() }
                            help={ Some(AttrValue::from(format!("Current value: {}", self.value_color))) } />
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

                        //<FormControl id="input-select-floating" ctype={ FormControlType::Select} class="mb-3"
                        //            help={ "Form control with floating label "} floating=true>
                        //   <SelectOption label="Selection an option" selected=true />
                        //   <SelectOption label="Option 1" value="1"/>
                        //   <SelectOption label="Option 2" value="2"/>
                        //   <SelectOption label="Option 3" value="3"/>
                        //<FormControl>
