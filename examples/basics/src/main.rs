use yew::prelude::*;

use yew_bootstrap::component::*;
use yew_bootstrap::icons::*;
use yew_bootstrap::util::*;
use gloo_console::debug;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

enum Msg {
    ToggleTooltip,
    ShowTooltip,
    HideTooltip,
}

struct Model {
    tooltip_show: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tooltip_show: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTooltip => {
                self.tooltip_show = !self.tooltip_show;
            }
            Msg::ShowTooltip => {
                self.tooltip_show = true;
            }
            Msg::HideTooltip => {
                self.tooltip_show = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let brand = BrandType::BrandIcon {
            text: AttrValue::from("Yew Bootstrap"),
            url: Some(AttrValue::from("https://yew.rs")),
            icon: BI::ROCKET,
        };

        // Show a message in the debug console whenever a NavItem or
        // NavDropdownItem is clicked.
        let onclick = Callback::from(move |event: MouseEvent| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(target) = target.dyn_into::<HtmlElement>() else {
                return;
            };
            debug!("onclick for:", target.inner_text().trim());
            // Stop the browser from actually following the "#" link.
            event.prevent_default();
        });

        let tooltip_click_p_ref = NodeRef::default();
        let tooltip_link_ref = NodeRef::default();

        html! {
            <>
                {include_inline()}
                {BIFiles::cdn()}
                <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-light bg-light" brand={brand}>
                    <NavItem text="link 1" icon={&BI::EMOJI_SUNGLASSES} onclick={onclick.clone()} url="#" />
                    <NavItem text="link 2" onclick={onclick.clone()} url="#" />
                    <NavDropdown text="several items" icon={&BI::MENU_APP}>
                        <NavDropdownItem text="Exclamation icon" icon={&BI::EXCLAMATION} onclick={onclick.clone()} url="#" />
                        <NavDropdownItem text="Magic icon" icon={&BI::MAGIC} onclick={onclick.clone()} url="#" />
                        <NavDropdownItem text="Tools icon" icon={&BI::TOOLS} onclick={onclick.clone()} url="#" />
                        <NavDropdownItem text="No icon" onclick={onclick.clone()} url="#" />
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
                    <h1>{ "Accordion" }</h1>
                    <Accordion>
                        <AccordionItem title={"Heading 1"}>
                            <p>{"Some text inside "}<strong>{"THE BODY"}</strong>{" of the accordion item"}</p>
                        </AccordionItem>
                        <AccordionItem title={"Heading 2"}>
                            <h3>{"Some other text under another accordion"}</h3>
                            <button>{"Button with some functionality"}</button>
                        </AccordionItem>
                    </Accordion>
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

                    <h1>{ "Badges" }</h1>
                    <Badge style={Color::Primary}>{"Primary"}</Badge>
                    <Badge style={Color::Secondary}>{"Secondary"}</Badge>
                    <Badge style={Color::Success}>{"Success"}</Badge>
                    <Badge style={Color::Danger}>{"Danger"}</Badge>
                    <Badge style={Color::Warning}>{"Warning"}</Badge>
                    <Badge style={Color::Info}>{"Info"}</Badge>
                    <Badge style={Color::Light}>{"Light"}</Badge>
                    <Badge style={Color::Dark}>{"Dark"}</Badge>

                    <h1>{ "Pill badges" }</h1>
                    <Badge style={Color::Primary} pill={true}>{"Primary"}</Badge>
                    <Badge style={Color::Secondary} pill={true}>{"Secondary"}</Badge>
                    <Badge style={Color::Success} pill={true}>{"Success"}</Badge>
                    <Badge style={Color::Danger} pill={true}>{"Danger"}</Badge>
                    <Badge style={Color::Warning} pill={true}>{"Warning"}</Badge>
                    <Badge style={Color::Info} pill={true}>{"Info"}</Badge>
                    <Badge style={Color::Light} pill={true}>{"Light"}</Badge>
                    <Badge style={Color::Dark} pill={true}>{"Dark"}</Badge>

                    <h1>{ "Positioned badges" }</h1>
                    <Button style={Color::Primary} class={"position-relative"}>
                        {"Primary"}
                        <Badge style={Color::Danger} position={(ArrangeX::Start100, ArrangeY::Top0)}>{"1"}</Badge>
                    </Button>

                    <h1>{"Border Spinner"}</h1>
                    <Spinner style={Color::Primary} />
                    <Spinner style={Color::Secondary} />
                    <Spinner style={Color::Success} />
                    <Spinner style={Color::Danger} />
                    <Spinner style={Color::Warning} />
                    <Spinner style={Color::Info} />
                    <Spinner style={Color::Light} />
                    <Spinner style={Color::Dark} />

                    <h1>{"Growing Spinner"}</h1>
                    <Spinner style={Color::Primary} grow={true} />
                    <Spinner style={Color::Secondary} grow={true} />
                    <Spinner style={Color::Success} grow={true} />
                    <Spinner style={Color::Danger} grow={true} />
                    <Spinner style={Color::Warning} grow={true} />
                    <Spinner style={Color::Info} grow={true} />
                    <Spinner style={Color::Light} grow={true} />
                    <Spinner style={Color::Dark} grow={true} />

                    <h1>{"Spinner on Buttons"}</h1>
                    <Button style={Color::Primary}>
                        <Spinner style={Color::Light} small={true} />
                        {"Loading..."}
                    </Button>
                    <Button style={Color::Primary}>
                        <Spinner style={Color::Light} grow={true} small={true} />
                        {"Loading..."}
                    </Button>

                    <h1>{"Display headings"}</h1>
                    <Display size={DisplaySize::One}>{"Display 1"}</Display>
                    <Display size={DisplaySize::Two}>{"Display 2"}</Display>
                    <Display size={DisplaySize::Three}>{"Display 3"}</Display>
                    <Display size={DisplaySize::Four}>{"Display 4"}</Display>
                    <Display size={DisplaySize::Five}>{"Display 5"}</Display>
                    <Display size={DisplaySize::Six}>{"Display 6"}</Display>

                    <h1>{"Lead paragraph"}</h1>
                    <Lead>
                        {"This is a lead paragraph. It stands out from regular paragraphs."}
                    </Lead>

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

                    <h2>{"Links which look like buttons"}</h2>
                    <Button style={Color::Primary} target={"_blank"} url={"https://github.com/isosphere/yew-bootstrap/"}>{"Primary link"}</Button>
                    <Button style={Color::Secondary} target={"_blank"} url={"https://github.com/isosphere/yew-bootstrap/"}>{"Secondary link"}</Button>
                    <Button style={Color::Primary} disabled={true} target={"_blank"} url={"https://github.com/isosphere/yew-bootstrap/"}>{"Disabled link"}</Button>

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

                    <h1>{"Links"}</h1>
                    <div class="d-grid gap-2">
                        <Link text={"Primary link"} style={Color::Primary} url={"https://github.com/isosphere/yew-bootstrap/"} />
                        <Link text={"Secondary link"} style={Color::Secondary} url={"https://github.com/isosphere/yew-bootstrap/"} />
                    </div>

                    <h1>{"List groups"}</h1>
                    <ListGroup>
                        <ListGroupItem>{"A"}</ListGroupItem>
                        <ListGroupItem active=true>{"B"}</ListGroupItem>
                        <ListGroupItem disabled=true>{"C"}</ListGroupItem>
                    </ListGroup>

                    <h2>{"Flush and with numbers"}</h2>
                    <ListGroup variant={ListGroupVariant::Flush} numbered=true>
                        <ListGroupItem style={Color::Primary}>{"Primary"}</ListGroupItem>
                        <ListGroupItem action=true style={Color::Secondary} url="#ref">{"Secondary w/ link"}</ListGroupItem>
                        <ListGroupItem action=true style={Color::Info}>{"Secondary w/ action"}</ListGroupItem>
                    </ListGroup>

                    <h2>{"Horizontal"}</h2>
                    <ListGroup horizontal={SizeTrigger::Always}>
                        <ListGroupItem action=true active=true>{"Active action"}</ListGroupItem>
                        <ListGroupItem action=true disabled=true>{"Disabled action"}</ListGroupItem>
                    </ListGroup>

                    <h1>{"Progress bars"}</h1>
                    <h2>{"Simple"}</h2>
                    <Progress class={"mb-3"}><ProgressBar value=0   /></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=25  /></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=50  /></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=75  /></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=100 /></Progress>

                    <h2>{"Labels"}</h2>
                    <Progress class={"mb-3"}><ProgressBar value=25 label={"25%"}/></Progress>

                    <h2>{"Height"}</h2>
                    <Progress class={"mb-3"} height=1><ProgressBar  value=25/></Progress>
                    <Progress class={"mb-3"} height=20><ProgressBar value=25/></Progress>

                    <h2>{"Backgrounds"}</h2>
                    <Progress class={"mb-3"}><ProgressBar value=25  style={Some(Color::Success)}/></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=50  style={Some(Color::Info)}/></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=75  style={Some(Color::Warning)}/></Progress>
                    <Progress class={"mb-3"}><ProgressBar value=100 style={Some(Color::Danger)}/></Progress>

                    <h2>{"Multiple bars"}</h2>
                    <Progress class={"mb-3"}>
                        <ProgressBar value=15/>
                        <ProgressBar value=30 style={Some(Color::Success)}/>
                        <ProgressBar value=20 style={Some(Color::Info)}/>
                    </Progress>

                    <h2>{"Striped"}</h2>
                    <Progress class={"mb-3"}><ProgressBar value=25 striped={true}/></Progress>

                    <h2>{"Animated"}</h2>
                    <Progress class={"mb-3"}><ProgressBar value=25 animated={true}/></Progress>

                    <h1>{"Tooltip"}</h1>
                    <p>
                        {"The "}
                        <Link
                            url="https://github.com/isosphere/yew-bootstrap/tree/main/examples/forms"
                            node_ref={tooltip_link_ref.clone()}
                            target="_blank"
                        >
                            <code>{"yew-bootstrap"}</code>
                            {" forms example"}
                        </Link>
                        {" demonstrates using a tooltip with many types of form control."}
                    </p>
                    <Tooltip
                        target={tooltip_link_ref}
                        placement={Placement::TopStart}
                        fade=true
                    >
                        {"Open the Forms example on "}{BI::GITHUB}{" GitHub"}
                    </Tooltip>
                    <h2>{"Buttons with tooltips (on focus or hover)"}</h2>
                    <ButtonGroup>
                        {
                            for [
                                (Color::Primary, Placement::Top),
                                (Color::Secondary, Placement::Bottom),
                                (Color::Success, Placement::Left),
                                (Color::Warning, Placement::Right),
                            ].iter().map(|(color, placement)| {
                                let btn_ref = NodeRef::default();

                                html_nested! {
                                    <>
                                        <Button style={color.clone()} node_ref={btn_ref.clone()}>
                                            {format!("Tooltip: {placement:?}")}
                                        </Button>
                                        <Tooltip target={btn_ref} placement={*placement}>
                                            {format!("Tooltip for button, placed at {placement:?}.")}
                                        </Tooltip>
                                    </>
                                }
                            })
                        }
                    </ButtonGroup>
                    <h2>{"Manually-triggered tooltip on an element"}</h2>
                    <p ref={tooltip_click_p_ref.clone()}>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt "}
                        {"ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation "}
                        {"ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in "}
                        {"reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur "}
                        {"sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id "}
                        {"est laborum."}
                    </p>
                    <ButtonGroup>
                        <Button onclick={ctx.link().callback(|_| Msg::ToggleTooltip)}>
                            {BI::TOGGLES}{" Toggle"}
                        </Button>
                        <Button onclick={ctx.link().callback(|_| Msg::ShowTooltip)}>
                            {BI::TOGGLE_ON}{" Show"}
                        </Button>
                        <Button onclick={ctx.link().callback(|_| Msg::HideTooltip)}>
                            {BI::TOGGLE_OFF}{" Hide"}
                        </Button>
                    </ButtonGroup>
                    <Tooltip
                        target={tooltip_click_p_ref}
                        trigger_on_focus=false
                        trigger_on_hover=false
                        show={self.tooltip_show}
                        placement={Placement::BottomEnd}
                    >
                        {"Tooltip toggled manually, targetted to the "}
                        <code>{"<p>"}</code>
                        {" tag."}
                    </Tooltip>
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
                { include_cdn_js() }
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
