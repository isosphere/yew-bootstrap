use yew::{html, virtual_dom::VNode};

pub fn init_cdn() -> VNode {
    html! {
        <link
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css"
            rel="stylesheet"
            integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3"
            crossorigin="anonymous"
        />
    }
}

pub fn init_inline() -> VNode {
    html! {
    }
}