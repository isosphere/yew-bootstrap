use yew::prelude::*;

use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

enum Msg {}
struct Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
                    <h1>{ "Accordian" }</h1>
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
                { include_cdn_js() }
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}