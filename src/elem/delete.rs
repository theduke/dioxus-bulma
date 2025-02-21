use dioxus::prelude::*;

use crate::Size;

#[derive(Props, PartialEq, Clone)]
pub struct DeleteProps {
    #[props(extends = GlobalAttributes, extends = button)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,

    pub onclick: EventHandler<Event<MouseData>>,
}

/// See: https://bulma.io/documentation/elements/delete/
pub fn Delete(props: DeleteProps) -> Element {
    let mut class = "delete".to_string();
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }
    rsx! {
        button {
            class,
            onclick: props.onclick,
            ..props.attributes,
        }
    }
}
