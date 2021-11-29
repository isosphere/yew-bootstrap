use crate::util::*;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Line {
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub height: Option<Size>,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub style: Option<Color>,

    #[prop_or_default]
    pub width: Option<Size>,
}

impl Component for Line {
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
            classes.push("vr");
        }
        if let Some(style) = self.props.style.clone() {
            classes.push(format!("bg-{}", style));
        }
        classes.push(self.props.class.clone());

        let mut css = String::new();
        if let Some(height) = self.props.height.clone() {
            css = format!("height: {}", height);
        }
        if let Some(width) = self.props.width.clone() {
            css = format!("{}; width: {}", css, width);
        }

        if self.props.vertical {
            html! {
                <div class=classes style=css />
            }
        } else {
            html! {
                <hr class=classes style=css />
            }
        }
    }
}
