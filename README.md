# yew-bootstrap
`yew-bootstrap` is a collection of frontend components made to simplify the usage of Bootstrap 5 within the Yew framework.

<a href="https://crates.io/crates/yew-bootstrap"><img alt="Crate info" src="https://img.shields.io/crates/v/yew-bootstrap.svg" /></a> 
<a href="https://docs.rs/yew-bootstrap/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-yew--bootstrap-green"/></a>

## Usage

This project assumes that you have an existing web application that uses the Yew framework.
If you do not, refer to [Yew Getting Started](https://yew.rs/getting-started/build-a-sample-app) to get started.

Add the dependency next to the regular yew dependency:

```toml
[dependencies]
yew = "0.20"
yew-bootstrap = "0.5.2"
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
`0.xy` 
Versions that have the same `x` value target the same version of Yew.

There is currently no indication of which version of Bootstrap is targeted, however, we’ll try to target the previous release pipeline. For example as of this writing the latest boostrap is 5.2.x, so we’ll try to target the latest 5.1.x version.

## Coverage

### Core Content

- [X] Container ([component::Container])
- [X] Grid ([component::Row], [component::Column])
- [ ] Display headings
- [ ] Lead
- [ ] Blockquote
- [ ] Image/Figure
- [ ] Table
- [ ] Forms

### Components

- [ ] Accordion
- [x] Alert ([component::Alert])
- [ ] Badge
- [ ] Breadcrumb
- [x] Button ([component::Button])
- [x] Button group ([component::ButtonGroup])
- [ ] Card
- [ ] Carousel
- [ ] Close button
- [ ] Collapse
- [ ] Dropdown
- [ ] List group
- [x] Modal
- [x] Navbar ([component::NavBar], [component::NavItem], [component::NavDropdown], [component::NavDropdownItem])
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
- [x] Colored links ([component::Link])
- [ ] Stacks
- [x] Stretched ([component::Link] with `stretched={true}>`)
- [ ] Text truncation
- [X] Vertical/Horizontal rule/line ([component::Line])
