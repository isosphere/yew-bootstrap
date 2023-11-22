use yew::prelude::*;

/// # Properties of [CardHeader]
#[derive(Properties, Debug, PartialEq)]
pub struct CardHeaderProps {
    /// Inner components (displayed in the [CardHeader])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

/// # Card Header component
/// A header inside of a [Card](super::Card).
///
/// See [CardHeaderProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card>
///         <CardHeader>{"Card Head"}</CardHeader>
///         <CardBody>{"Body text"}</CardBody>
///         <CardFooter>{"Card Foot"}</CardFooter>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardHeader(props: &CardHeaderProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-header");

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

/// # Properties of [CardBody]
#[derive(Properties, Debug, PartialEq)]
pub struct CardBodyProps {
    /// Inner components (displayed in the [CardBody])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

/// # Card Body component
/// A body inside of a [Card](super::Card). This may be implicitly added using the `body` prop in a
/// [Card](super::Card)
///
/// See [CardBodyProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card>
///         <CardHeader>{"Card Head"}</CardHeader>
///         <CardBody>{"Body text"}</CardBody>
///         <CardFooter>{"Card Foot"}</CardFooter>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardBody(props: &CardBodyProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-body");

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

/// # Properties of [CardFooter]
#[derive(Properties, Debug, PartialEq)]
pub struct CardFooterProps {
    /// Inner components (displayed in the [CardFooter])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

/// # Card Footer component
/// A footer inside of a [Card](super::Card).
///
/// See [CardFooterProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card>
///         <CardHeader>{"Card Head"}</CardHeader>
///         <CardBody>{"Body text"}</CardBody>
///         <CardFooter>{"Card Foot"}</CardFooter>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardFooter(props: &CardFooterProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-footer");

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

/// # Properties of [CardTitle]
#[derive(Properties, Debug, PartialEq)]
pub struct CardTitleProps {
    /// Inner components (displayed in the [CardTitle])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

/// # Card Title component
/// Title text inside of a [CardBody].
///
/// See [CardTitleProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card>
///         <CardBody>
///             <CardTitle>{"Title text"}</CardTitle>
///             <CardSubtitle>{"Subtitle text"}</CardSubtitle>
///             <CardText>{"Generic text"}</CardText>
///         </CardBody>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardTitle(props: &CardTitleProps) -> Html {
    let mut classes = props.class.clone();
    classes.extend(["card-title", "h5"]);

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

/// # Properties of [CardSubtitle]
#[derive(Properties, Debug, PartialEq)]
pub struct CardSubtitleProps {
    /// Inner components (displayed in the [CardSubtitle])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

/// # Card Subtitle component
/// Subtitle text inside of a [CardBody].
///
/// See [CardSubtitleProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card>
///         <CardBody>
///             <CardTitle>{"Title text"}</CardTitle>
///             <CardSubtitle>{"Subtitle text"}</CardSubtitle>
///             <CardText>{"Generic text"}</CardText>
///         </CardBody>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardSubtitle(props: &CardSubtitleProps) -> Html {
    let mut classes = props.class.clone();
    classes.extend(["mb-2", "text-muted", "card-subtitle", "h6"]);

    html!{
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

/// # Properties of [CardText]
#[derive(Properties, Debug, PartialEq)]
pub struct CardTextProps {
    /// Inner components (displayed in the [CardText])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

/// # Card Title component
/// Default paragraph text inside of a [CardBody].
///
/// See [CardTextProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card>
///         <CardBody>
///             <CardTitle>{"Title text"}</CardTitle>
///             <CardSubtitle>{"Subtitle text"}</CardSubtitle>
///             <CardText>{"Generic text"}</CardText>
///         </CardBody>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardText(props: &CardTextProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-text");

    html!{
        <p class={classes}>
            {props.children.clone()}
        </p>
    }
}

/// # Properties of [CardLink]
#[derive(Properties, Debug, PartialEq)]
pub struct CardLinkProps {
    /// Inner components (displayed in the [CardLink])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
    /// URL that the link points to
    pub url: AttrValue,
}

/// # Card Title component
/// Title text inside of a [CardBody].
///
/// See [CardLinkProps] for a property list.
///
/// ## Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_bootstrap::component::card::*;
/// fn test() -> Html {
///   html! {
///     <Card body=true>
///         <CardLink url="foo">{"Link text"}</CardLink>
///     </Card>
///   }
/// }
/// ```
#[function_component]
pub fn CardLink(props: &CardLinkProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-link");

    html!{
        <a class={classes} href={props.url.clone()}>
            {props.children.clone()}
        </a>
    }
}
