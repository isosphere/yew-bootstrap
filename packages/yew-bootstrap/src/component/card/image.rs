use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq]
pub enum ImageVariant {
    Default,
    Top,
    Bottom,
}

#[derive(Properties, Debug, PartialEq)]
pub struct CardImageProps {
    #[prop_or(ImageVariant::Default)]
    pub variant: ImageVariant,
    #[prop_or_default]
    pub class: Classes,
    pub src: AttrValue,
    #[prop_or_default]
    pub alt: AttrValue,
}

// #[function_component]
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
