use yew::prelude::*;

pub struct Modal { }

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub title: String,
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
            <div class="modal" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">{props.title.clone()}</h5>
                        </div>
                        <div class="modal-body">
                            { for props.children.iter() }
                        </div>
                        <div class="modal-footer">
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}