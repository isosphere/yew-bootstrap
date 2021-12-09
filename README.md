# yew-bootstrap

`yew-bootstrap` is a collection of frontend components made to simplify the usage of Bootstrap within the Yew framework.

## Usage

This project assumes that you have an existing web application that uses the Seed framework.
If you do not, refer to the [Yew Getting Started](https://yew.rs/getting-started/build-a-sample-app) project to get started.

Add the dependency next to the regular yew dependency:

```toml
[dependencies]
yew = "0.18"
yew-bootstrap = "0.3"
```

Then in the beginning of your application, include the `include_cdn()` or `include_inline()` function to load the required CSS and JS, either from JSDeliver CDN or to inline the CSS:

```Rust
    fn view(&self) -> Html {
        html! {
            <>
                {include_cdn()}
                <Button style=Color::Primary>{"Primary"}</Button>
            </>
        }
    }
```

## Coverage

Currently missing the following Core content:

- [X] Container (`<Container>`)
- [X] Grid (`<Row>`, `<Column>`)
- [ ] Display headings
- [ ] Lead
- [ ] Blockquote
- [ ] Image/Figure
- [ ] Table
- [ ] Forms

Following Components has been implemented:

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

The following Helpers has been implemented:

- [ ] Clearfix
- [x] Colored links (`<Link>`)
- [ ] Stacks
- [x] Stretched (`<Link stretched=true>`)
- [ ] Text truncation
- [X] Vertical/Horizontal rule/line (`<Line>`)