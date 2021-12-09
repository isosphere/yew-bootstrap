use log::warn;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Column {
    props: ComponentProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(Some(0))]
    pub size: Option<u8>,

    #[prop_or_default]
    pub sm: Option<u8>,

    #[prop_or_default]
    pub md: Option<u8>,

    #[prop_or_default]
    pub lg: Option<u8>,

    #[prop_or_default]
    pub xl: Option<u8>,

    #[prop_or_default]
    pub xxl: Option<u8>,
}

impl Component for Column {
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
        if self.props.size.unwrap_or(0) > 12 {
            warn!("Column `size` cannot be greater than 12");
        }
        if self.props.sm.unwrap_or(0) > 12 {
            warn!("Column `sm` size cannot be greater than 12");
        }
        if self.props.md.unwrap_or(0) > 12 {
            warn!("Column `md` size cannot be greater than 12");
        }
        if self.props.lg.unwrap_or(0) > 12 {
            warn!("Column `lg` size cannot be greater than 12");
        }
        if self.props.xl.unwrap_or(0) > 12 {
            warn!("Column `xl` size cannot be greater than 12");
        }
        if self.props.xxl.unwrap_or(0) > 12 {
            warn!("Column `xxl` size cannot be greater than 12");
        }
        let mut classes = Classes::new();
        if let Some(size) = self.props.size {
            if size == 0 {
                classes.push("col");
            } else {
                classes.push("col-".to_string() + &size.to_string());
            }
        }
        if let Some(sm) = self.props.sm {
            classes.push("col-sm-".to_string() + &sm.to_string());
        }
        if let Some(md) = self.props.md {
            classes.push("col-md-".to_string() + &md.to_string());
        }
        if let Some(lg) = self.props.lg {
            classes.push("col-lg-".to_string() + &lg.to_string());
        }
        if let Some(xl) = self.props.xl {
            classes.push("col-xl-".to_string() + &xl.to_string());
        }
        if let Some(xxl) = self.props.xxl {
            classes.push("col-xxl-".to_string() + &xxl.to_string());
        }
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
