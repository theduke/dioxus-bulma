use dioxus::prelude::*;

use crate::Size;

#[derive(Props, PartialEq, Clone)]
pub struct SectionProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    size: Option<Size>,

    children: Element,
}

/// See: https://bulma.io/documentation/layout/section/
pub fn Section(props: SectionProps) -> Element {
    let mut class = "section".to_string();
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }
    rsx! {
        div {
            class,
            ..props.attributes,
            {props.children}
        }
    }
}
