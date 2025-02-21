use yew::prelude::*;
use yew_bootstrap::util::{include_inline, include_cdn_js};

mod single;
mod multiple;
mod headers;
mod form;
mod filter;

use single::*;
use multiple::*;
use headers::*;
use form::*;
use filter::*;

#[function_component(App)]
fn app() -> Html {
    // Example list of items
    let items = [
        AttrValue::from("Apple"),
        AttrValue::from("Banana"),
        AttrValue::from("Cherry"),
        AttrValue::from("Date"),
        AttrValue::from("Elderberry"),
        AttrValue::from("Fig"),
        AttrValue::from("Grapes"),
    ];

    let items_with_headers = [
        AttrValue::from("Apple"),
        AttrValue::from("Header 1"),
        AttrValue::from("Banana"),
        AttrValue::from("Cherry"),
        AttrValue::from("Date"),
        AttrValue::from("Header 2"),
        AttrValue::from("Elderberry"),
        AttrValue::from("Fig"),
        AttrValue::from("Grapes"),
    ];

    html! {
        <>
            { include_inline() }
            <Single items={items.to_vec()} />
            <Multiple items={items.to_vec()} />
            <Headers items={items_with_headers.to_vec()} />
            <Form items={items.to_vec()} />
            <Filter items={items.to_vec()} />
            <p style="height: 400px;"></p>
            { include_cdn_js() }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

