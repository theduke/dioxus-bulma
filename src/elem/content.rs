use dioxus::prelude::*;

use crate::Size;

#[derive(Props, PartialEq, Clone)]
pub struct ContentProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,
}

/// See: https://bulma.io/documentation/elements/content/
pub fn Content(props: ContentProps) -> Element {
    let mut class = "content".to_string();
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }
    rsx! {
        div {
            class,
            ..props.attributes
        }
    }
}
