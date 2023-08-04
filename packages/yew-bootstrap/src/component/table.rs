use std::fmt::Display;
use yew::prelude::*;

use crate::util::Color;

/// # TableHeaderScope enum
#[derive(Clone, PartialEq, Eq)]
pub enum TableHeaderScope {
    /// Header is for a column
    Col,
    /// Header is for a row
    Row,
    /// Header is for a group of columns
    ColGroup,
    /// Header is for a group of rows
    RowGroup,
}

impl Display for TableHeaderScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableHeaderScope::Col => write!(f, "col"),
            TableHeaderScope::Row => write!(f, "row"),
            TableHeaderScope::ColGroup => write!(f, "colgroup"),
            TableHeaderScope::RowGroup => write!(f, "rowgroup"),
        }
    }
}

/// # TableHeader component; i.e.: `<th>`.
pub struct TableHeader {}

/// # Properties for [TableHeader]
#[derive(Properties, Clone, PartialEq)]
pub struct TableHeaderProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// children
    #[prop_or_default]
    pub children: Children,

    /// Scope, used primarily to assist screen readers
    #[prop_or_default]
    pub scope: Option<TableHeaderScope>,

    /// table [color variant](https://getbootstrap.com/docs/5.1/content/tables/#variants)
    #[prop_or_default]
    pub color: Option<Color>,    
}

impl Component for TableHeader {
    type Message = ();
    type Properties = TableHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();    

        let mut classes = Classes::new();
        classes.push(props.class.clone());

        if let Some(color) = props.color.clone() {
            classes.push(format!("table-{}", color));
        }

        html!{
            if let Some(scope) = props.scope.clone() {
                <th class={classes} scope={format!("{}", scope)}>
                    { for props.children.iter() }
                </th>
            } else {
                <th class={classes}>
                    { for props.children.iter() }
                </th>
            }
        }
    }
}

/// # TableHead component; i.e.: `<thead>`.
pub struct TableHead {}

/// # Properties for [TableHead]
#[derive(Properties, Clone, PartialEq)]
pub struct TableHeadProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// children
    #[prop_or_default]
    pub children: Children,
}

impl Component for TableHead {
    type Message = ();
    type Properties = TableHeadProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();    

        let mut classes = Classes::new();
        classes.push(props.class.clone());

        html!{
            <thead class={classes}>
                { for props.children.iter() }
            </thead>
        }
    }
}


/// # TableRow component; i.e.: `<tr>`.
pub struct TableRow {}

/// # Properties for [TableRow]
#[derive(Properties, Clone, PartialEq)]
pub struct TableRowProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// children
    #[prop_or_default]
    pub children: Children,

    /// table [color variant](https://getbootstrap.com/docs/5.1/content/tables/#variants)
    #[prop_or_default]
    pub color: Option<Color>,
}

impl Component for TableRow {
    type Message = ();
    type Properties = TableRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();    

        let mut classes = Classes::new();
        classes.push(props.class.clone());

        if let Some(color) = props.color.clone() {
            classes.push(format!("table-{}", color));
        }

        html!{
            <tr class={classes}>
                { for props.children.iter() }
            </tr>
        }
    }
}

/// # TableBody component; i.e.: `<tbody>`.
pub struct TableBody {}

/// # Properties for [TableBody]
#[derive(Properties, Clone, PartialEq)]
pub struct TableBodyProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// children
    #[prop_or_default]
    pub children: Children,
}

impl Component for TableBody {
    type Message = ();
    type Properties = TableBodyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();    

        let mut classes = Classes::new();
        classes.push(props.class.clone());

        html!{
            <tbody class={classes}>
                { for props.children.iter() }
            </tbody>
        }
    }
}

/// # TableCell component; i.e.: `<td>`.
pub struct TableCell {}

/// # Properties for [TableCell]
#[derive(Properties, Clone, PartialEq)]
pub struct TableCellProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// children
    #[prop_or_default]
    pub children: Children,

    /// column span (i.e.: `colspan` attribute)
    #[prop_or_default]
    pub col_span: Option<usize>,

    /// table [color variant](https://getbootstrap.com/docs/5.1/content/tables/#variants)
    #[prop_or_default]
    pub color: Option<Color>,
}

impl Component for TableCell {
    type Message = ();
    type Properties = TableCellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();    

        let mut classes = Classes::new();
        classes.push(props.class.clone());

        if let Some(color) = props.color.clone() {
            classes.push(format!("table-{}", color));
        }

        html!{
            if let Some(col_span) = props.col_span {
                <td class={classes} colspan={format!("{}", col_span)}>
                    { for props.children.iter() }
                </td>
            } else {
                <td class={classes}>
                    { for props.children.iter() }
                </td>
            }
        }
    }
}

/// # Table component
pub struct Table {}

/// # Properties for [Table]
#[derive(Properties, Clone, PartialEq)]
pub struct TableProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// children
    #[prop_or_default]
    pub children: Children,

    /// table [color variant](https://getbootstrap.com/docs/5.1/content/tables/#variants)
    #[prop_or_default]
    pub color: Option<Color>,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();    

        let mut classes = Classes::new();
        classes.push(props.class.clone());

        if let Some(color) = props.color.clone() {
            classes.push(format!("table-{}", color));
        } else {
            classes.push("table");
        }

        html!{
            <table class={classes}>
                { for props.children.iter() }
            </table>
        }
    }
}