use yew::prelude::*;
use super::Container;
use crate::util::Dimension;


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
    pub children: Children,
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
                    { for props.children.iter() }
                </ul>
            </li>
        }
    }
}

pub struct NavItem { }

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

    /// required for dropdowns
    #[prop_or_default]
    pub id: String,

    /// dropdown items
    #[prop_or_default]
    pub children: Children
}

impl Component for NavItem {
    type Message = ();
    type Properties = NavItemProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        match &props.children.is_empty() {
            true => {
                let mut classes = Classes::new();
                classes.push(String::from("nav-link"));

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
                    <NavDropdown text={props.text.clone()} id={props.id.clone()}>
                        { for props.children.iter() }
                    </NavDropdown>
                }                
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum BrandType {
    BrandSimple { text: String, url: Option<String> },
    BrandImage { 
        /// browser-accessible url to the brand image
        image_url: String, 
        /// descriptive text for screen reader users
        alt: String, 
        dimension: Option<Dimension>
    },
    BrandCombined {
        text: String, 
        /// hyperlink destination for brand text
        url: Option<String>,
        /// browser-accessible url to the brand image
        image_url: String, 
        /// descriptive text for screen reader users
        alt: String, 
        dimension: Option<Dimension>
    }
}

pub struct NavBar { }

#[derive(Properties, Clone, PartialEq)]
pub struct NavBarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,

    /// the id of the div that contains the nav-items
    #[prop_or_default]
    pub nav_id: String,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub brand: Option<BrandType>
}

impl Component for NavBar {
    type Message = ();
    type Properties = NavBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let expanded = String::from(match &props.expanded {
            true => {
                "true"
            },
            false => {
                "false"
            }
        });

        let mut classes = Classes::new();
        classes.push("navbar");
        classes.push(props.class.clone());

        let brand = match &props.brand {
            None => html!{},
            Some(b) => {
                match b {
                    BrandType::BrandSimple{text, url} => {
                        let url = match url { 
                            Some(u) => u.clone(),
                            None => String::from("#")
                        };

                        html!{
                            <a class="navbar-brand" href={url}>
                                {text.clone()}
                            </a>
                        }
                    },
                    BrandType::BrandImage { image_url, alt, dimension } => {
                        match dimension {
                            None => {
                                html! {
                                    <a class="navbar-brand" href={"#"}>
                                        <img src={image_url.clone()} alt={alt.clone()} class="d-inline-block align-text-top" />
                                    </a>
                                }
                            }
                            Some(Dimension{width, height}) => {
                                html! {
                                    <a class="navbar-brand" href={"#"}>
                                        <img src={image_url.clone()} alt={alt.clone()} width={width.clone()} height={height.clone()} class="d-inline-block align-text-top" />
                                    </a>
                                }
                            }
                        }
                    }
                    BrandType::BrandCombined { text, url, image_url, alt, dimension } => {
                        let url = match url { 
                            Some(u) => u.clone(),
                            None => String::from("#")
                        };
                        match dimension {
                            None => {
                                html! {
                                    <a class="navbar-brand" href={url}>
                                        <img src={image_url.clone()} alt={alt.clone()} class="d-inline-block align-text-top" />
                                        {text.clone()}
                                    </a>
                                }
                            },
                            Some(Dimension{width, height}) => {
                                html! {
                                    <a class="navbar-brand" href={url}>
                                        <img src={image_url.clone()} alt={alt.clone()} width={width.clone()} height={height.clone()} class="d-inline-block align-text-top" />
                                        {text.clone()}
                                    </a>
                                }
                            }
                        }
                    }
                }
            }
        };

        html! {
            <nav class={classes}>
                <Container fluid=true>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target={format!("#{}", props.nav_id.clone())} aria-controls={props.nav_id.clone()} aria-expanded={expanded} aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    {brand}
                    <div class="collapse navbar-collapse" id={props.nav_id.clone()}>
                        <ul class="navbar-nav">
                            { for props.children.clone() }
                        </ul>
                    </div>
                </Container>
            </nav>
        }
    }
}