## Usage

This project assumes that you have an existing web application that uses the [Yew framework](https://yew.rs/).

Add the dependency next to the regular yew dependency:

```toml
[dependencies]
yew = "0.20"
yew-bootstrap = "0.5"
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
`0.x.y` 
Versions that have the same `x` value target the same version of Yew.

There is currently no indication of which version of Bootstrap is targeted, however, we’ll try to target the previous release pipeline. For example as of this writing the latest boostrap is 5.2.x, so we’ll try to target the latest 5.1.x version.

## Coverage

### Core Content

- [X] Container ([component::Container])
- [X] Grid ([component::Row], [component::Column])
- [x] Display headings ([component::Display])
- [ ] Lead
- [ ] Blockquote
- [ ] Image/Figure
- [ ] Table
- [x] Forms ([component::form::FormControl])

### Components

- [ ] Accordion
- [x] Alert ([component::Alert])
- [x] Badge ([component::Badge])
- [ ] Breadcrumb
- [x] Button ([component::Button])
- [x] Button group ([component::ButtonGroup])
- [ ] Card
- [ ] Carousel
- [ ] Close button
- [ ] Collapse
- [ ] Dropdown
- [ ] List group
- [x] Modal ([component::Modal])
- [x] Navbar ([component::NavBar], [component::NavItem], [component::NavDropdown], [component::NavDropdownItem])
- [ ] Navs & tabs
- [ ] Offcanvas
- [ ] Pagination
- [ ] Placeholders
- [ ] Popovers
- [ ] Progress
- [ ] Scrollspy
- [x] Spinner ([component::Spinner])
- [ ] Toast
- [ ] Tooltips

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
