# yew-bootstrap

`yew-bootstrap` is a collection of frontend components made to simplify the usage of Bootstrap 5 within the Yew framework.

## Usage

This project assumes that you have an existing web application that uses the Yew framework.
If you do not, refer to [Yew Getting Started](https://yew.rs/getting-started/build-a-sample-app) to get started.

Add the dependency next to the regular yew dependency:

```toml
[dependencies]
yew = "0.20"
yew-bootstrap = { git = "https://github.com/dataheck/yew-bootstrap" }
```

Then in the beginning of your application, include the `include_cdn()` or `include_inline()` function to load the required CSS and JS, either from JSDeliver CDN or to inline the CSS:

```Rust
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                {include_cdn()}
                <Button style={Color::Primary}>{"Primary"}</Button>
            </>
        }
    }
```

## Coverage

### Core Content

- [X] Container (`<Container>`)
- [X] Grid (`<Row>`, `<Column>`)
- [ ] Display headings
- [ ] Lead
- [ ] Blockquote
- [ ] Image/Figure
- [ ] Table
- [ ] Forms

### Components

- [ ] Accordion
- [x] Alert (`<Alert>`)
- [ ] Badge
- [ ] Breadcrumb
- [x] Button (`<Button>`)
- [x] Button group (`<ButtonGroup>`)
- [ ] Card
- [ ] Carousel
- [ ] Close button
- [ ] Collapse
- [ ] Dropdown
- [ ] List group
- [ ] Modal
- [ ] Navs & tabs
- [ ] Offcanvas
- [ ] Pagination
- [ ] Placeholders
- [ ] Popovers
- [ ] Progress
- [ ] Scrollspy
- [ ] Spinner
- [ ] Toast
- [ ] Tooltips

### Helpers

- [ ] Clearfix
- [x] Colored links (`<Link>`)
- [ ] Stacks
- [x] Stretched (`<Link stretched={true}>`)
- [ ] Text truncation
- [X] Vertical/Horizontal rule/line (`<Line>`)
