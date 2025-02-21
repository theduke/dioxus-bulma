use dioxus::prelude::*;

use crate::{Color, Delete, Size};

#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub header: Option<Element>,
    pub body: Element,

    pub color: Option<Color>,
    pub size: Option<Size>,
    pub on_delete: Option<EventHandler<Event<MouseData>>>,
}

/// See: https://bulma.io/documentation/components/message/
pub fn Message(props: MessageProps) -> Element {
    debug_assert!(
        if props.header.is_some() {
            props.on_delete.is_some()
        } else {
            true
        },
        "A bulma Message with a delete button must have a header."
    );

    let mut class = "message".to_string();
    if let Some(color) = props.color {
        class.push(' ');
        class.push_str(color.class());
    }
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }

    rsx! {
        div {
            class,
            ..props.attributes,
            if let Some(header) = props.header {
                div {
                    class: "message-header",
                    { header }

                    if let Some(on_delete) = props.on_delete {
                        Delete {
                            onclick: on_delete,
                        }
                    }
                }
            }
            div {
                class: "message-body",
                { props.body}
            }
        }
    }
}
