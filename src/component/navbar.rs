use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct NavBarBrandImage {
    pub url: String,
    /// descriptive text for users of screen readers
    pub alt: String,
    pub width: Option<String>,
    pub height: Option<String>,
}


pub struct NavBarBrand { }

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct NavBarBrandProperties {
    #[prop_or_default]
    pub text: String,
    
    /// Optional brand image
    #[prop_or_default]
    pub image: Option<NavBarBrandImage>,

    #[prop_or_default]
    pub url: Option<String>,
}

impl Component for NavBarBrand { 
    type Message = ();
    type Properties = NavBarBrandProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let url = match &props.url { 
            Some(u) => u.clone(),
            None => String::from("#")
        };

        match &props.image {
            Some(i) => {
                match (&i.width, &i.height) {
                    (None, _) | (_, None)=> {
                        html! {
                            <a class="navbar-brand" href={url}>
                                <img src={i.url.clone()} alt={i.alt.clone()} class="d-inline-block align-text-top" />
                                {props.text.clone()}
                            </a>
                        }
                    },
                    (Some(w), Some(h)) => {
                        html! {
                            <a class="navbar-brand" href={url}>
                                <img src={i.url.clone()} alt={i.alt.clone()} width={w.clone()} height={h.clone()} class="d-inline-block align-text-top" />
                                {props.text.clone()}
                            </a>
                        }
                    }
                }
            },
            None => {
                html! {
                    <a class="navbar-brand" href={url}>
                        {props.text.clone()}
                    </a>
                }
            }
        }
    }
} 