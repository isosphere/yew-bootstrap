use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum HeadingSize {
    H1,
    H2,
    H3,
    H4,
    H5,
    P,
}

#[derive(Properties, Clone, PartialEq)]
struct AccordionHeaderProps {
    #[prop_or(HeadingSize::H2)]
    heading_size: HeadingSize,
    #[prop_or_default]
    heading_id: String,
    #[prop_or_default]
    children: Children,
}

#[function_component]
fn AccordionHeader(props: &AccordionHeaderProps) -> Html {
    match props.heading_size {
        HeadingSize::H1 => {
            html! { <h1 class="accordion-header" id={props.heading_id.clone()}>{ for props.children.iter() }</h1> }
        },
        HeadingSize::H2 => {
            html! { <h2 class="accordion-header" id={props.heading_id.clone()}>{ for props.children.iter() }</h2> }
        }
        HeadingSize::H3 => {
            html! { <h3 class="accordion-header" id={props.heading_id.clone()}>{ for props.children.iter() }</h3> }
        }
        HeadingSize::H4 => {
            html! { <h4 class="accordion-header" id={props.heading_id.clone()}>{ for props.children.iter() }</h4> }
        }
        HeadingSize::H5 => {
            html! { <h5 class="accordion-header" id={props.heading_id.clone()}>{ for props.children.iter() }</h5> }
        }
        HeadingSize::P => {
            html! { <p class="accordion-header" id={props.heading_id.clone()}>{ for props.children.iter() }</p> }
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionItemProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or(HeadingSize::H2)]
    pub heading_size: HeadingSize,
    #[prop_or(String::from("main-accordion"))]
    parent_id: String,
    #[prop_or_default]
    item_id: usize,
    #[prop_or_default]
    pub expanded: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn AccordionItem(props: &AccordionItemProps) -> Html {
    let heading_id = format!("{}-heading-{}", props.parent_id, props.item_id);
    let collapse_id = format!("{}-collapse-{}", props.parent_id, props.item_id);

    let mut button_classes = classes!("accordion-button");
    let mut collapse_classes = classes!("accordion-collapse",  "collapse");
    if !props.expanded {
        button_classes.push("collapsed");
    } else {
        collapse_classes.push("show");
    }

    html! {
        <div class="accordion-item">
            <AccordionHeader heading_size={props.heading_size.clone()} heading_id={heading_id.clone()}>
                <button class={button_classes} type="button" data-bs-toggle="collapse" data-bs-target={format!("#{}", collapse_id)} aria-expanded={props.expanded.to_string()} aria-controls={collapse_id.clone()}>
                    { props.title.clone() }
                </button>
            </AccordionHeader>
            <div id={collapse_id.clone()} class={collapse_classes} aria-labelledby={heading_id.clone()} data-bs-parent={format!("#{}", props.parent_id)}>
                <div class="accordion-body">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    #[prop_or(String::from("main-accordion"))]
    pub id: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<AccordionItem>,
}

#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    html!{
        <div class="accordian" id={props.id.clone()}>
            {
                for props.children.iter().enumerate().map(|(index, mut child)| {
                    let child_props = Rc::make_mut(&mut child.props);
                    child_props.item_id = index;
                    child_props.parent_id = props.id.clone();
                    child
                })
            }
        </div>
    }
}