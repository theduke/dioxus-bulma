use dioxus::prelude::*;

use crate::Color;

#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    #[props(extends = GlobalAttributes, extends = table)]
    attributes: Vec<Attribute>,

    pub color: Option<Color>,
    #[props(default = false)]
    pub bordered: bool,
    #[props(default = false)]
    pub striped: bool,
    #[props(default = false)]
    pub narrow: bool,
    #[props(default = false)]
    pub hoverable: bool,
    #[props(default = false)]
    pub fullwidth: bool,

    pub thead: Option<Element>,
    pub tfoot: Option<Element>,
    pub tbody: Option<Element>,
}

/// See: https://bulma.io/documentation/elements/table/
pub fn Table(props: TableProps) -> Element {
    let mut class = "table".to_string();

    if let Some(color) = props.color {
        class.push(' ');
        class.push_str(color.class());
    }
    if props.bordered {
        class.push_str(" is-bordered");
    }
    if props.striped {
        class.push_str(" is-striped");
    }
    if props.narrow {
        class.push_str(" is-narrow");
    }
    if props.hoverable {
        class.push_str(" is-hoverable");
    }
    if props.fullwidth {
        class.push_str(" is-fullwidth");
    }

    rsx! {
        table {
            class,
            ..props.attributes,

            if let Some(thead) = props.thead {
                thead {
                    {thead}
                }
            }

            if let Some(tfoot) = props.tfoot {
                tfoot {
                    {tfoot}
                }
            }

            if let Some(tbody) = props.tbody {
                tbody {
                    {tbody}
                }
            }
        }
    }
}

/// Table that is horizontally scrollable.
///
/// Wraps a table in a div with the class "table-container".
///
/// Inherited props from [`Table`]: see [`TableProps`].
///
/// See: https://bulma.io/documentation/elements/table/
pub fn ScrollableTable(props: TableProps) -> Element {
    rsx! {
        div {
            class: "table-container",
            Table {
                ..props
            }
        }
    }
}
