use dioxus::prelude::*;

use crate::{Color, Delete};

#[derive(Props, PartialEq, Clone)]
pub struct NotificationProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub color: Option<Color>,

    /// If provided, adds a "delete" button to the notification and
    /// connects this event handler to the button's "click" event.
    pub on_delete: Option<EventHandler<Event<MouseData>>>,

    pub children: Element,
}

/// See: https://bulma.io/documentation/elements/notification/
pub fn Notification(props: NotificationProps) -> Element {
    let mut class = "notification".to_string();
    if let Some(color) = props.color {
        class.push(' ');
        class.push_str(color.class());
    }

    rsx! {
        div {
            class,
            ..props.attributes,

            if let Some(on_delete) = props.on_delete {
                Delete {
                    onclick: on_delete,
                }
            }

            {props.children}
        }
    }
}
