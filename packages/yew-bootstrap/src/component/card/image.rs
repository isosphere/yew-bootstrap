use yew::prelude::*;

/// Controls the display variant used for a [CardImage]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageVariant {
    /// Default display. Nothing special.
    Default,
    /// Image at the top of a card.
    Top,
    /// Image at the bottom of a card.
    Bottom,
}

/// # Properties of [CardImage]
#[derive(Properties, Debug, PartialEq)]
pub struct CardImageProps {
    /// The display variant to use. See [ImageVariant] for more details.
    #[prop_or(ImageVariant::Default)]
    pub variant: ImageVariant,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
    /// The image source to use. Passed directly to the `src` property of the generated `img`.
    pub src: AttrValue,
    /// Descriptive text for screen reader users.
    #[prop_or_default]
    pub alt: AttrValue,
}

#[function_component]
pub fn CardImage(props: &CardImageProps) -> Html {
    let mut classes = match props.variant {
        ImageVariant::Default => Classes::from("card-img"),
        ImageVariant::Top => Classes::from("card-img-top"),
        ImageVariant::Bottom => Classes::from("card-img-bottom"),
    };

    classes.extend(&props.class);

    html! {
        <img class={classes} data-src={props.src.clone()} style="height: 180px; width: 100%; display: block;" alt={props.alt.clone()} />
    }
}

/// # Properties of [CardImageOverlay]
#[derive(Properties, Debug, PartialEq)]
pub struct CardImageOverlayProps {
    /// Inner components (displayed in the [CardImageOverlay])
    #[prop_or_default]
    pub children: Children,
    /// Extra CSS classes to include, in addition to the defaults.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn CardImageOverlay(props: &CardImageOverlayProps) -> Html {
    let mut classes = props.class.clone();
    classes.push("card-img-overlay");

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}
