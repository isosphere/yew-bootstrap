## Usage

This project assumes that you have an existing web application that uses the [Yew framework](https://yew.rs/).

Add the dependency next to the regular yew dependency:

```toml
[dependencies]
yew = "0.21"
yew-bootstrap = "*"
```

To use form callback functions, the following dependencies should be added:

```toml
[dependencies]
wasm-bindgen = "0.2.*"
web-sys = { version = "0.3.*", features = ["HtmlTextAreaElement", "HtmlSelectElement"] }
```

Then in the beginning of your application, include the `include_cdn()` or `include_inline()` function to load the required CSS. Some components require the Bootstrap JavaScript
library to be loaded - for these you can use the `include_cdn_js()` function. It is recommended that you put this at the bottom of your `html!{}` macro, as done below:

```Rust
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                {include_cdn()}
                <Button style={Color::Primary}>{"Primary"}</Button>
                {include_cdn_js()}
            </>
        }
    }
```

Check `main.rs` for example usage for every implemented component.

## Version Convention
This project uses [semantic versioning](https://semver.org/).

## Coverage

### Core Content

- [X] Container ([component::Container])
- [X] Grid ([component::Row], [component::Column])
- [x] Display headings ([component::Display])
- [x] Lead ([component::Lead])
- [ ] Blockquote
- [ ] Image/Figure
- [ ] Table
- [x] Forms ([component::form::FormControl])

### Components

- [x] Accordion ([component::Accordion])
- [x] Alert ([component::Alert])
- [x] Badge ([component::Badge])
- [ ] Breadcrumb
- [x] Button ([component::Button])
- [x] Button group ([component::ButtonGroup])
- [x] Card ([component::Card], [component::CardGroup])
- [ ] Carousel
- [ ] Close button
- [ ] Collapse
- [ ] Dropdown
- [x] List group ([component::ListGroup], [component::ListGroupItem])
- [x] Modal ([component::Modal])
- [x] Navbar ([component::NavBar], [component::NavItem], [component::NavDropdown], [component::NavDropdownItem])
- [ ] Navs & tabs
- [ ] Offcanvas
- [ ] Pagination
- [ ] Placeholders
- [ ] Popovers
- [x] Progress ([component::Progress], [component::ProgressBar])
- [ ] Scrollspy
- [x] Spinner ([component::Spinner])
- [ ] Toast
- [x] Tooltips ([component::Tooltip])

### Helpers

- [ ] Clearfix
- [x] Colored links ([component::Link])
- [ ] Stacks
- [x] Stretched ([component::Link] with `stretched={true}>`)
- [ ] Text truncation
- [X] Vertical/Horizontal rule/line ([component::Line])

## Examples

Several examples are provided:

- `examples/basics`: Components
- `examples/forms`: Form fields

To run an example:

```bash
cd examples/<directory>
trunk --serve
```
