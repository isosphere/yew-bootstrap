use std::rc::Rc;

use yew::prelude::*;

/// # Properties of [AccordionHeader]
#[derive(Properties, Clone, PartialEq)]
struct AccordionHeaderProps {
    /// The html id of this component
    #[prop_or_default]
    heading_id: AttrValue,

    /// The title displayed in the header
    #[prop_or_default]
    title: AttrValue,

    /// Classes attached to the button holding the title
    #[prop_or_default]
    button_classes: Classes,

    /// The html id of associated collapse for this [AccordionItem]
    #[prop_or_default]
    collapse_id: AttrValue,

    /// If the associated accordion collapse is open
    #[prop_or_default]
    expanded: bool
}

/// # Accordion Header
/// Used with [crate::component::AccordionItem] to create accordion drop downs
/// This represents the title of the accordion item that is always visible
/// 
/// See [AccordionHeaderProps] for a listing of properties
///
/// This component is not meant to be used stand-alone as it's only rendered inside of Accordions
#[function_component]
fn AccordionHeader(props: &AccordionHeaderProps) -> Html {
    html! { 
        <h2 class="accordion-header" id={props.heading_id.clone()}>
            <button
                class={props.button_classes.clone()} 
                type="button" 
                data-bs-toggle="collapse" 
                data-bs-target={format!("#{}", props.collapse_id)} 
                aria-expanded={props.expanded.to_string()} 
                aria-controls={props.collapse_id.clone()}
            >
                { props.title.clone() }
            </button>
        </h2>
    }
}

/// # Properties of [AccordionCollapse]
#[derive(Properties, Clone, PartialEq)]
struct AccordionCollapseProps {
    /// Parent [Accordion] html id attribute
    #[prop_or(AttrValue::from("main-accordion"))]
    parent_id: AttrValue,

    /// Html id of this component
    #[prop_or_default]
    collapse_id: AttrValue,

    /// Html id of associated header for this [AccordionItem]
    #[prop_or_default]
    heading_id: AttrValue,

    /// Opening this item will close other items in the [Accordion]
    #[prop_or_default]
    stay_open: bool,

    /// Classes attached to the div
    #[prop_or_default]
    class: Classes,

    /// Inner components
    #[prop_or_default]
    children: Children,
}

/// # Accordion Collapse
/// Used with [crate::component::AccordionItem] to create accordion drop downs
/// This represents the body of the accordion item that can be opened/closed
/// 
/// See [AccordionCollapseProps] for a listing of properties
///
/// This component is not meant to be used stand-alone as it's only rendered inside of Accordions
#[function_component]
fn AccordionCollapse(props: &AccordionCollapseProps) -> Html {
    if props.stay_open {
        return html! {
            <div id={props.collapse_id.clone()} class={props.class.clone()} aria-labelledby={props.heading_id.clone()}>
                { for props.children.iter() }
            </div>
        }
    }
    html! {
        <div id={props.collapse_id.clone()} class={props.class.clone()} aria-labelledby={props.heading_id.clone()} data-bs-parent={format!("#{}", props.parent_id)}>
            { for props.children.iter() }
        </div>
    }
}

/// # Properties of [AccordionItem]
#[derive(Properties, Clone, PartialEq)]
pub struct AccordionItemProps {
    /// Text displayed in this items heading
    #[prop_or_default]
    pub title: AttrValue,

    /// Item is currently open
    #[prop_or_default]
    pub expanded: bool,

    /// Inner components (displayed in the [AccordionCollapse])
    #[prop_or_default]
    pub children: Children,

    /// Opening this item doesn't close other items
    #[prop_or_default]
    stay_open: bool,

    /// Html id attribute of parent [Accordion]
    #[prop_or(AttrValue::from("main-accordion"))]
    parent_id: AttrValue,

    /// Position in the parent [Accordion]
    #[prop_or_default]
    item_id: usize,
}

/// # A singular accordion item, child of [Accordion]
/// Used as a child of [Accordion] to create an accordion menu.
/// 
/// Child components will be displayed in the body of the accordion item
#[function_component]
pub fn AccordionItem(props: &AccordionItemProps) -> Html {
    let heading_id = format!("{}-heading-{}", props.parent_id, props.item_id);
    let collapse_id = format!("{}-collapse-{}", props.parent_id, props.item_id);

    let mut button_classes = classes!("accordion-button");
    let mut collapse_classes = classes!("accordion-collapse",  "collapse");

    // TODO: Maybe hook up the `expanded` property to some state depending on `stay_open`
    //
    // I think in the bootstrap docs this is really only meant to show one item as expanded after loading the page
    // However as it currently is, users may be able to set this on multiple items at once
    // This is probably fine during initial page load since they can be closed individually
    // But it acts weird if an end-user were to open another item as it would close all of them unless `stay_open` is true
    // 
    // Additionally if some other part of the page is setup to use state to open an item
    // This will cause 2 items to be open at once even if the `stay_open` flag is false
    // There's no real harm putting the closing of accordion items on the user, but it would be nice if there were
    // some sort of built in way to handle this
    //
    // I use ssr in my project so ideally this would also not interfere with rendering server side
    if !props.expanded {
        button_classes.push("collapsed");
    } else {
        collapse_classes.push("show");
    }

    html! {
        <div class="accordion-item">
            <AccordionHeader 
                title={props.title.clone()}
                heading_id={heading_id.clone()}
                button_classes={button_classes}
                collapse_id={collapse_id.clone()}
                expanded={props.expanded}
            />
            <AccordionCollapse
                class={collapse_classes}
                stay_open={props.stay_open}
                heading_id={heading_id}
                collapse_id={collapse_id.clone()}
                parent_id={props.parent_id.clone()}
            >
                <div class="accordion-body">
                    { for props.children.iter() }
                </div>
            </AccordionCollapse>
        </div>
    }
}

/// # Properties of [Accordion]
#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    /// Html id of the accordion - should be unique within it's page
    #[prop_or(AttrValue::from("main-accordion"))]
    pub id: AttrValue,

    /// Accordion is flush with the container and removes some styling elements
    #[prop_or_default]
    pub flush: bool,

    /// Opening an item won't close other items in the accordion
    #[prop_or_default]
    pub stay_open: bool,

    // The [AccordionItem] instances controlled by this accordion
    #[prop_or_default]
    pub children: ChildrenWithProps<AccordionItem>,
}

/// # Accordion
/// [Accordion] is used to group several [crate::component::AccordionItem] instances together.
/// 
/// See [AccordionProps] for a listing of properties.
/// 
/// See [bootstrap docs](https://getbootstrap.com/docs/5.0/components/accordion/) for a full demo of accordions
/// 
/// Basic example of using an Accordion
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Accordion, AccordionItem};
/// fn test() -> Html {
///     html!{
///         <Accordion>
///             <AccordionItem title={"Heading 1"}>
///                 <p>{"Some text inside "}<strong>{"THE BODY"}</strong>{" of the accordion item"}</p>
///             </AccordionItem>
///             <AccordionItem title={"Heading 2"}>
///                 <h3>{"Some other text under another accordion"}</h3>
///                 <button>{"Button with some functionality"}</button>
///             </AccordionItem>
///         </Accordion>
///     }
/// }
/// ```
/// 
/// 
/// Example of using an Accordion while mapping a list to AccordionItem children
/// 
/// ```rust
/// use yew::{prelude::*, virtual_dom::VChild};
/// use yew_bootstrap::component::{Accordion, AccordionItem};
/// fn test() -> Html {
///     let items = vec![("title1", "body1"), ("title2", "body2")];
///     html! {
///         <Accordion id="features-and-challenges">
///             {
///                 items.iter().map(|item| {
///                     html_nested! {
///                         <AccordionItem title={item.0.clone()}>
///                             {item.0.clone()}
///                         </AccordionItem>
///                     }
///                 }).collect::<Vec<VChild<AccordionItem>>>()
///             }
///         </Accordion>
///     }
/// }
/// ```
#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    let mut classes = classes!("accordion");
    if props.flush {
        classes.push("accordion-flush");
    }

    html! {
        <div class={classes} id={props.id.clone()}>
            {
                for props.children.iter().enumerate().map(|(index, mut child)| {
                    let child_props = Rc::make_mut(&mut child.props);
                    child_props.item_id = index;
                    child_props.parent_id = props.id.clone();
                    child_props.stay_open = props.stay_open;
                    child
                })
            }
        </div>
    }
}