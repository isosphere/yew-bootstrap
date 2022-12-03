use yew::{html, virtual_dom::VNode};

pub fn include_cdn() -> VNode {
    html! {
        <link
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css"
            rel="stylesheet"
            integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3"
            crossorigin="anonymous"
        />
    }
}


pub fn include_cdn_js() -> VNode {
    html! {
        <>
            <link data-trunk={"true"} rel="copy-file" href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js.map" />
            <script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js"
                data-trunk={"true"}
                integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3"
                crossorigin="anonymous"
            >
            </script>
        </>
    }
}


pub fn include_inline() -> VNode {
    html! {
        <style>
            {include_str!("bootstrap-5.1.3.min.css")}
        </style>
    }
}