use yew::prelude::*;
use yew_bootstrap::icons::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"I"}<span style="color: red">{BI::HEART_FILL}</span>{BI::GEAR_FILL}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
