use crate::util::Color;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct ButtonGroup{
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub role: String,

    #[prop_or_default]
    pub vertical: bool,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = ComponentProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::new();
        if self.props.vertical {
            classes.push("btn-group-vertical");
        } else {
            classes.push("btn-group");
        }
        classes.push(self.props.class.clone());

        html! {
            <div
                class=classes
                role=self.props.role.clone()
                aria-label=self.props.label.clone()
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
