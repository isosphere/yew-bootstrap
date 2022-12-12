use yew::prelude::*;
use super::Container;
use crate::util::Dimension;


/// # A singular dropdown item, child of [NavDropdown]
/// Used as a child of [NavDropdown] to create a dropdown menu. See [NavDropdownItemProps] for a listing of properties.
pub struct NavDropdownItem { }

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct NavDropdownItemProps {
    #[prop_or_default]
    pub text: AttrValue,
    #[prop_or_default]
    pub url: AttrValue,
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

/// A dropdown menu, child of [NavBar]. See [NavDropdownProps] for a listing of properties.
#[derive(Clone, PartialEq, Eq)]
pub struct NavDropdown { }

#[derive(Properties, Clone, PartialEq)]
pub struct NavDropdownProps {
    #[prop_or_default]
    pub children: Children,
    /// the id of the link with the dropdown-toggle class, referenced by aria-labelledby
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub expanded: bool,
    /// the text of the link with the dropdown-toggle class
    #[prop_or_default]
    pub text: AttrValue
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
    pub url: Option<AttrValue>,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// ignored if dropdown is Some
    #[prop_or_default]
    pub text: AttrValue,

    /// required for dropdowns
    #[prop_or_default]
    pub id: AttrValue,

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
    BrandSimple { 
        text: AttrValue, url: Option<AttrValue> },
    /// a brand icon is a bootstrap icon, requiring bootstrap-icons to be imported; see [crate::util::include_cdn_icons]
    BrandIcon { icon: AttrValue, text: AttrValue, url: Option<AttrValue> },
    BrandImage { 
        /// browser-accessible url to the brand image
        image_url: AttrValue, 
        /// descriptive text for screen reader users
        alt: AttrValue, 
        dimension: Option<Dimension>
    },
    BrandCombined {
        text: AttrValue, 
        /// hyperlink destination for brand text
        url: Option<AttrValue>,
        /// browser-accessible url to the brand image
        image_url: AttrValue, 
        /// descriptive text for screen reader users
        alt: AttrValue, 
        dimension: Option<Dimension>
    }
}

/// # Navbar component, parent of [NavItem], [NavDropdown], and [NavDropdownItem]
/// The navbar is a responsive horizontal menu bar that can contain links, dropdowns, and text.
/// We have broken up this component into several sub-components to make it easier to use: [NavItem], [NavDropdown], and [NavDropdownItem].
/// The brand property is set using the [BrandType] enum.
/// 
/// See [NavBarProps] for more information on properties supported by this component.
/// # Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{BrandType, NavBar, NavDropdownItem, NavItem};
/// 
/// fn test() -> Html {
///     let brand = BrandType::BrandSimple { 
///         text: AttrValue::from("Yew Bootstrap"), 
///         url: Some(AttrValue::from("https://yew.rs")) 
///     };
///     html!{
///         <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-light bg-light" brand={brand}>
///             <NavItem text="Home" url={AttrValue::from("/")} />
///             <NavItem text="more">
///                 <NavDropdownItem text="dropdown item 1" url={AttrValue::from("/dropdown1")} />
///             </NavItem>
///         </NavBar>
///     }
/// }
/// ```
pub struct NavBar { }

#[derive(Properties, Clone, PartialEq)]
pub struct NavBarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,

    /// the id of the div that contains the nav-items
    #[prop_or_default]
    pub nav_id: AttrValue,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub brand: Option<BrandType>,

    #[prop_or_default]
    pub brand_callback: Callback<MouseEvent>
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
        classes.push(props.class.to_string());

        let brand = match &props.brand {
            None => html!{},
            Some(b) => {
                match b {
                    BrandType::BrandSimple{text, url} => {
                        let url = match url { 
                            Some(u) => u.clone(),
                            None => AttrValue::from("#")
                        };

                        html!{
                            <a class="navbar-brand" href={url} onclick={props.brand_callback.clone()}>
                                {text.clone()}
                            </a>
                        }
                    },
                    BrandType::BrandIcon { text, icon, url } => {
                        let url = match url { 
                            Some(u) => u.clone(),
                            None => AttrValue::from("#")
                        };
                        html! {
                            <a class="navbar-brand" href={url} onclick={props.brand_callback.clone()}>
                                <i class={format!("bi-{}", icon)}></i>
                                {text.clone()}
                            </a>
                        }
                    }
                    BrandType::BrandImage { image_url, alt, dimension } => {
                        match dimension {
                            None => {
                                html! {
                                    <a class="navbar-brand" href={"#"} onclick={props.brand_callback.clone()}>
                                        <img src={image_url.clone()} alt={alt.clone()} class="d-inline-block align-text-top" />
                                    </a>
                                }
                            }
                            Some(Dimension{width, height}) => {
                                html! {
                                    <a class="navbar-brand" href={"#"} onclick={props.brand_callback.clone()}>
                                        <img src={image_url.clone()} alt={alt.clone()} width={width.clone()} height={height.clone()} class="d-inline-block align-text-top" />
                                    </a>
                                }
                            }
                        }
                    }
                    BrandType::BrandCombined { text, url, image_url, alt, dimension } => {
                        let url = match url { 
                            Some(u) => u.clone(),
                            None => AttrValue::from("#")
                        };
                        match dimension {
                            None => {
                                html! {
                                    <a class="navbar-brand" href={url} onclick={props.brand_callback.clone()}>
                                        <img src={image_url.clone()} alt={alt.clone()} class="d-inline-block align-text-top" />
                                        {text.clone()}
                                    </a>
                                }
                            },
                            Some(Dimension{width, height}) => {
                                html! {
                                    <a class="navbar-brand" href={url} onclick={props.brand_callback.clone()}>
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