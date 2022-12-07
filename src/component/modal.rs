use yew::prelude::*;

pub struct ModalFooter { }

#[derive(Properties, Clone, PartialEq)]
pub struct ModalFooterProps {
    #[prop_or_default]
    pub children: Children
}

impl Component for ModalFooter {
    type Message = ();
    type Properties = ModalFooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="modal-footer">
                { for props.children.iter() }
            </div>
        }
    }
}


pub struct ModalHeader { }

#[derive(Properties, Clone, PartialEq)]
pub struct ModalHeaderProps {
    #[prop_or_default]
    pub title: String,
    /// required for triggering open/close
    #[prop_or_default]
    pub id: String,
}

impl Component for ModalHeader {
    type Message = ();
    type Properties = ModalHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="modal-header">
                <h5 class="modal-title" id={format!("#{}", props.id.clone())}>{props.title.clone()}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
        }
    }
}


pub struct ModalBody { }

#[derive(Properties, Clone, PartialEq)]
pub struct ModalBodyProps {
    #[prop_or_default]
    pub children: Children
}

impl Component for ModalBody {
    type Message = ();
    type Properties = ModalBodyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="modal-body">
                { for props.children.iter() }
            </div>
        }
    }
}


pub struct Modal { }

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub title: String,
    /// required for triggering open/close
    #[prop_or_default]
    pub id: String,
    /// modal body
    #[prop_or_default]
    pub children: Children
}

impl Component for Modal {
    type Message = ();
    type Properties = ModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="modal" tabindex="-1" id={props.id.clone()}>
                <div class="modal-dialog">
                    <div class="modal-content">
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        }
    }
}