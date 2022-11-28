use yew::prelude::*;
use super::Container;

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

pub struct NavDropdownItem { }

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct NavDropdownItemProps {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub url: String,
}

impl Component for NavDropdownItem {
    type Message = ();
    type Properties = NavDropdownItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <li>
                <a class="dropdown-item" href={props.url.clone()}>{props.text.clone()}</a>
            </li>
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct NavDropdown { }

#[derive(Properties, Clone, PartialEq)]
pub struct NavDropdownProps {
    #[prop_or_default]
    pub items: Children,
    /// the id of the link with the dropdown-toggle class, referenced by aria-labelledby
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub expanded: bool,
    #[prop_or_default]
    pub text: String
}

impl Component for NavDropdown {
    type Message = ();
    type Properties = NavDropdownProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let expanded = String::from(match props.expanded {
            true => "true",
            false => "false"
        });

        html! {
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" id={props.id.clone()} role="button" data-bs-toggle="dropdown" aria-expanded={expanded}>
                    {props.text.clone()}
                </a>
                <ul class="dropdown-menu" aria-labelledby={props.id.clone()}>
                    { for props.items.iter() }
                </ul>
            </li>
        }
    }
}

pub struct NavLinkItem { }

#[derive(Properties, Clone, PartialEq)]
pub struct NavItemProperties {
    #[prop_or_default]
    pub url: Option<String>,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// ignored if dropdown is Some
    #[prop_or_default]
    pub text: String,

    #[prop_or_default]
    pub dropdown: Children
}

impl Component for NavLinkItem {
    type Message = ();
    type Properties = NavItemProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        match &props.dropdown.is_empty() {
            true => {
                let mut classes = Classes::new();

                if props.active {
                    classes.push(String::from("active"));
                }

                if props.disabled {
                    classes.push(String::from("disabled"));
                }

                match props.disabled {
                    true => {
                        html! {
                            <li class="nav-item">
                                <a class={classes} tabindex="-1" aria-disabled="true" href={props.url.clone()}>
                                    {props.text.clone()}
                                </a>
                            </li>
                        }
                    },
                    false => {
                        html! {
                            <li class="nav-item">
                                <a class={classes} href={props.url.clone()}>
                                    {props.text.clone()}
                                </a>
                            </li>
                        }
                    }
                }
            },
            false => {
                html! {
                    { for props.dropdown.iter() }
                }                
            }
        }
    }
}

pub struct NavBar { }

#[derive(Properties, Clone, PartialEq)]
pub struct NavBarProps {
    #[prop_or_default]
    pub items: Children,
    #[prop_or_default]
    pub brand: Children,
    #[prop_or_default]
    pub class: String,

    /// the id of the div that contains the nav-items
    #[prop_or_default]
    pub nav_id: String,

    #[prop_or_default]
    pub expanded: bool
}

impl Component for NavBar {
    type Message = ();
    type Properties = NavBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        // todo: use our container component?
        let expanded = String::from(match &props.expanded {
            true => {
                "true"
            },
            false => {
                "false"
            }
        });

        html! {
            <nav class={props.class.clone()}>
                <div class="container-fluid">
                    { for props.brand.clone() }
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target={format!("#{}", props.nav_id.clone())} aria-controls={props.nav_id.clone()} aria-expanded={expanded} aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id={props.nav_id.clone()}>
                        { for props.items.clone() }
                    </div>
                </div>
            </nav>
        }
    }
}