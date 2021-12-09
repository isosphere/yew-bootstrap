use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

enum Msg {
    #[allow(dead_code)]
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    _link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        /*
        <div>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            <p>{ self.value }</p>
        </div>
        */
        html! {
            <>
                {include_inline()}
                <div id="layout" class="p-3">
                    <h1>{ "Containers" }</h1>
                    <Container class="bg-primary">{"Normal"}</Container>
                    <Container class="bg-secondary" fluid=true>{"Fluid"}</Container>
                    <Container class="bg-success" size=ContainerSize::Small>{"Small"}</Container>
                    <Container class="bg-danger" size=ContainerSize::Medium>{"Medium"}</Container>
                    <Container class="bg-warning" size=ContainerSize::Large>{"Large"}</Container>
                    <Container class="bg-info" size=ContainerSize::ExtraLarge>{"Extra Large"}</Container>
                    <Container class="bg-light" size=ContainerSize::ExtraExtraLarge>{"Extra Large"}</Container>
                </div>
                <div id="components" class="p-3">
                    <h1>{ "Alerts" }</h1>
                    <Alert style=Color::Primary>
                        { "This is a primary alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Secondary>
                        { "This is a secondary alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Success>
                        { "This is a success alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Danger>
                        { "This is a danger alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Warning>
                        { "This is a warning alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Info>
                        { "This is a info alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Light>
                        { "This is a light alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Dark>
                        { "This is a dark alert—check it out!" }
                    </Alert>
                    <Alert style=Color::Link>
                        { "This is a link alert—check it out!" }
                    </Alert>

                    <h1>{"Buttons"}</h1>
                    <Button style=Color::Primary>{"Primary"}</Button>
                    <Button style=Color::Secondary>{"Secondary"}</Button>
                    <Button style=Color::Success>{"Success"}</Button>
                    <Button style=Color::Danger>{"Danger"}</Button>
                    <Button style=Color::Warning>{"Warning"}</Button>
                    <Button style=Color::Info>{"Info"}</Button>
                    <Button style=Color::Light>{"Light"}</Button>
                    <Button style=Color::Dark>{"Dark"}</Button>
                    <Button style=Color::Link>{"Link"}</Button>

                    <h2>{"Outline buttons"}</h2>
                    <Button style=Color::Primary outline=true>{"Primary"}</Button>
                    <Button style=Color::Secondary outline=true>{"Secondary"}</Button>
                    <Button style=Color::Success outline=true>{"Success"}</Button>
                    <Button style=Color::Danger outline=true>{"Danger"}</Button>
                    <Button style=Color::Warning outline=true>{"Warning"}</Button>
                    <Button style=Color::Info outline=true>{"Info"}</Button>
                    <Button style=Color::Light outline=true>{"Light"}</Button>
                    <Button style=Color::Dark outline=true>{"Dark"}</Button>
                    <Button style=Color::Link outline=true text="Link2" />

                    <h2>{"Sizes"}</h2>
                    <Button style=Color::Primary size=ButtonSize::Large>{"Large button"}</Button>
                    <Button style=Color::Secondary size=ButtonSize::Large>{"Large button"}</Button>
                    <br />
                    <Button style=Color::Primary size=ButtonSize::Normal>{"Normal button"}</Button>
                    <Button style=Color::Secondary size=ButtonSize::Normal>{"Normal button"}</Button>
                    <br />
                    <Button style=Color::Primary size=ButtonSize::Small>{"Small button"}</Button>
                    <Button style=Color::Secondary size=ButtonSize::Small>{"Small button"}</Button>

                    <h2>{"Disabled state"}</h2>
                    <Button style=Color::Primary disabled=true>{"Primary"}</Button>
                    <Button style=Color::Secondary disabled=true>{"Secondary"}</Button>

                    <h2>{"Block buttons"}</h2>
                    <div class="d-grid gap-2">
                        <Button style=Color::Primary block=true>{"Primary"}</Button>
                        <Button style=Color::Secondary block=true>{"Secondary"}</Button>
                    </div>

                    <h1>{"Button groups"}</h1>
                    <ButtonGroup>
                        <Button style=Color::Primary>{"Primary"}</Button>
                        <Button style=Color::Secondary>{"Secondary"}</Button>
                    </ButtonGroup>

                    <h2>{"Vertical variation"}</h2>
                    <ButtonGroup vertical=true>
                        <Button style=Color::Primary>{"Primary"}</Button>
                        <Button style=Color::Secondary>{"Secondary"}</Button>
                    </ButtonGroup>
                </div>
                <div id="helpers" class="p-3">
                    <h1>{"Vertical/Horizontal rule"}</h1>
                    <h2>{"Horizontal"}</h2>
                    <Line />
                    <Line style=Color::Primary />
                    <Line height=Size::Px(5) />
                    <Line width=Size::Px(100) />
                    <h2>{"Vertical"}</h2>
                    <Line vertical=true /><br />
                    <Line vertical=true style=Color::Primary /><br />
                    <Line vertical=true height=Size::Px(50) /><br />
                    <Line vertical=true width=Size::Px(100) /><br />
                </div>
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn main() {
    yew::start_app::<Model>();
}
