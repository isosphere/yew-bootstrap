use yew::prelude::*;

use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
use yew_bootstrap::component::card::*;

enum Msg {}

struct Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let all_colors = [
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Info,
            Color::Warning,
            Color::Danger,
            Color::Light,
            Color::Dark,
        ];

        let bg_cards = all_colors.iter()
            .map(|color| {
                let text_color = if *color == Color::Dark {
                    Some(TextColor::White)
                } else {
                    None
                };

                html_nested! {
                <Column>
                    <Card bg={color.clone()} text={text_color}>
                        <CardHeader>{"Header"}</CardHeader>
                        <CardBody>
                            <CardTitle>{"Primary Title"}</CardTitle>
                            <CardText>{"Some example text to make the card a little bigger, provide it some more content."}</CardText>
                        </CardBody>
                    </Card>
                </Column>
                }
            })
            .collect::<Vec<_>>();

        let border_cards = all_colors.iter()
            .map(|color| html_nested! {
                <Column>
                    <Card border={color.clone()}>
                        <CardHeader>{"Header"}</CardHeader>
                        <CardBody>
                            <CardTitle>{"Primary Title"}</CardTitle>
                            <CardText>{"Some example text to make the card a little bigger, provide it some more content."}</CardText>
                        </CardBody>
                    </Card>
                </Column>
            })
            .collect::<Vec<_>>();

        html! {
            <>
                {include_inline()}
                <div id="layout" class="p-3">

                <h1>{"Basic Cards"}</h1>

                <h2>{"Card with inline body"}</h2>
                <Card body=true class="w-25">
                    {"A very simple card"}
                </Card>

                <h2>{"Card with header and footer"}</h2>
                <Card class="w-25">
                    <CardHeader>{"This is the card header"}</CardHeader>
                    <CardBody>
                        {"This is the body of the card"}
                    </CardBody>
                    <CardFooter>{"This is the card footer"}</CardFooter>
                </Card>

                <h2>{"Card with title/subtitle and links"}</h2>
                <Card body=true class="w-25">
                    <CardTitle>{"Title"}</CardTitle>
                    <CardSubtitle>{"Subtitle"}</CardSubtitle>

                    <CardText>{"Card text, blah blah blah"}</CardText>

                    <CardLink url="#link1">{"Link 1"}</CardLink>
                    <CardLink url="#link2">{"Link 2"}</CardLink>
                </Card>

                <h2>{"Card with a list and header"}</h2>
                <Card class="w-25">
                    <CardHeader>{"Header Text"}</CardHeader>
                    <ListGroup variant={ListGroupVariant::Flush}>
                        <ListGroupItem>{"First Item"}</ListGroupItem>
                        <ListGroupItem>{"Second Item"}</ListGroupItem>
                        <ListGroupItem>{"Third Item"}</ListGroupItem>
                        <ListGroupItem>{"Fourth Item"}</ListGroupItem>
                    </ListGroup>
                </Card>

                <h1>{"Cards with images"}</h1>
                <Row>
                    <Column>
                        <Card>
                            <CardText>{"Above Text"}</CardText>
                            <CardImage src="TODO"/>
                            <CardText>{"Below Text"}</CardText>
                        </Card>
                    </Column>
                    <Column>
                        <Card>
                            <CardImage variant={ImageVariant::Top} src="TODO"/>
                            <CardText>{"Below Text"}</CardText>
                        </Card>
                    </Column>
                    <Column>
                        <Card>
                            <CardText>{"Above Text"}</CardText>
                            <CardImage variant={ImageVariant::Bottom} src="TODO"/>
                        </Card>
                    </Column>
                </Row>

                <h1>{"Card colors"}</h1>
                <h2>{"Background colors"}</h2>
                <Row class="g-4">
                    {bg_cards}
                </Row>

                <h2>{"Border colors"}</h2>
                <Row class="g-4">
                    {border_cards}
                </Row>

                <h1>{"Card groups"}</h1>
                <CardGroup>
                    <Card>
                        <CardHeader>{"This card has a header"}</CardHeader>
                        <CardBody>
                            <CardTitle>{"Card title"}</CardTitle>
                            <CardText>
                                {"This is a wider card with supporting text below as a natural lead-in \
                                to additional content. This content is a little bit longer."}
                            </CardText>
                        </CardBody>
                        <CardFooter>
                            <small className="text-muted">{"Last updated 3 mins ago"}</small>
                        </CardFooter>
                    </Card>
                    <Card>
                        <CardBody>
                            <CardTitle>{"Card title"}</CardTitle>
                            <CardText>
                                {"This card has supporting text below as a natural lead-in to \
                                additional content. "}
                            </CardText>
                        </CardBody>
                        <CardFooter>
                            <small className="text-muted">{"Last updated 3 mins ago"}</small>
                        </CardFooter>
                    </Card>
                    <Card>
                        <CardBody>
                            <CardTitle>{"Card title"}</CardTitle>
                            <CardText>
                                {"This is a wider card with supporting text below as a natural lead-in \
                                to additional content. This card has even longer content than the \
                                first to show that equal height action. We need even more text for that \
                                so here's some more text to use. Even more words to push the text amount \
                                up even more. Another line or two should do it, enough to counter the header \
                                on the first card."}
                            </CardText>
                        </CardBody>
                        <CardFooter>
                            <small className="text-muted">{"Last updated 3 mins ago"}</small>
                        </CardFooter>
                    </Card>
                </CardGroup>

                </div>
                { include_cdn_js() }
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}