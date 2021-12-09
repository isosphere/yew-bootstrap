use super::Column;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Row {
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: ChildrenWithProps<Column>,
}

impl Component for Row {
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
        classes.push("row");
        classes.push(self.props.class.clone());

        html! {
            <div
                class=classes
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
