use yew::prelude::*;

/// Represents the optional size of a Modal dialog, described [here](https://getbootstrap.com/docs/5.1/components/modal/#optional-sizes)
#[derive(Clone, PartialEq, Eq)]
pub enum ModalSize {
    ExtraLarge,
    Large,
    Normal,
    Small,
}

impl Default for ModalSize {
    fn default() -> Self {
        ModalSize::Normal
    }
}

/// Properties for [ModalFooter]
#[derive(Properties, Clone, PartialEq)]
pub struct ModalFooterProps {
    #[prop_or_default]
    pub children: Children
}

/// # Footer for a [Modal] dialog
/// See [ModalFooterProps] for a listing of properties
#[function_component]
pub fn ModalFooter(props: &ModalFooterProps) -> Html {
    html! {
        <div class="modal-footer">
            { for props.children.iter() }
        </div>
    }
}

/// Properties for [ModalHeader]
#[derive(Properties, Clone, PartialEq)]
pub struct ModalHeaderProps {
    /// Title for the Modal dialog
    #[prop_or_default]
    pub title: String,

    /// required for triggering open/close
    #[prop_or_default]
    pub id: String,
}

/// # Header for a [Modal] dialog
/// See [ModalHeaderProps] for a listing of properties
///
#[function_component]
pub fn ModalHeader(props: &ModalHeaderProps) -> Html {
    html! {
        <div class="modal-header">
            <h5 class="modal-title" id={format!("#{}", props.id.clone())}>{props.title.clone()}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
    }
}

/// Properties for [ModalBody]
#[derive(Properties, Clone, PartialEq)]
pub struct ModalBodyProps {
    #[prop_or_default]
    pub children: Children
}


/// # Body for a [Modal] dialog
/// See [ModalBodyProps] for a listing of properties
#[function_component]
pub fn ModalBody(props: &ModalBodyProps) -> Html {
    html! {
        <div class="modal-body">
            { for props.children.iter() }
        </div>
    }
}

/// Properties for Modal
#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub title: String,
    /// required for triggering open/close
    #[prop_or_default]
    pub id: String,
    /// modal body, typically [ModalHeader], [ModalBody] or [ModalFooter]
    #[prop_or_default]
    pub children: Children,
    /// Size of the modal
    #[prop_or_default]
    pub size: ModalSize,
}

/// # Modal dialog
/// Modal dialog, parent of [ModalHeader], [ModalBody] and [ModalFooter].
///
/// See [ModalProps] for a listing of properties
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Modal, ModalHeader, ModalBody, ModalFooter, Button, ModalSize};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Modal id="ExampleModal" size={ModalSize::Large}> // size defaults to Normal
///             <ModalHeader title="Modal title" id="ExampleModal"/>
///             <ModalBody>
///                 <p>{"Modal body text goes here."}</p>
///             </ModalBody>
///             <ModalFooter>
///                 <Button style={ Color::Secondary } modal_dismiss={ true }>{ "Close" }</Button>
///                 <Button style={ Color::Primary }>{ "Save changes" }</Button>
///             </ModalFooter>
///         </Modal>
///     }
/// }
/// ```
#[function_component]
pub fn  Modal(props: &ModalProps) -> Html {
    let mut dialog_classes = Classes::new();
    dialog_classes.push("modal-dialog");

    match props.size {
        ModalSize::ExtraLarge => dialog_classes.push("modal-xl"),
        ModalSize::Large => dialog_classes.push("modal-lg"),
        ModalSize::Small => dialog_classes.push("modal-sm"),
        _ => (),
    }

    html! {
        <div class="modal" tabindex="-1" id={props.id.clone()}>
            <div class={dialog_classes}>
                <div class="modal-content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}