use popper_rs::prelude::Placement;
use yew::prelude::*;

bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct OverlayTriggerType : u8 {
        const HOVER = 0x1;
        const CLICK = 0x2;
        const FOCUS = 0x4;
    }
}

// https://github.com/react-bootstrap/react-bootstrap/blob/master/src/OverlayTrigger.tsx
pub struct OverlayTrigger {}

/// # Properties for [OverlayTrigger]
#[derive(Properties, Clone, PartialEq)]
pub struct OverlayTriggerProps {
    /// Optional children
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub trigger: OverlayTriggerType,
    
    #[prop_or_default]
    pub show: bool,

    /// Placement of the tooltip.
    ///
    /// [Popper's website shows all placement options][0].
    ///
    /// [0]: https://popper.js.org/
    #[prop_or_default]
    pub placement: Placement,
}

impl Component for OverlayTrigger {
    type Message = ();
    type Properties = OverlayTriggerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            
        }

    }
}
