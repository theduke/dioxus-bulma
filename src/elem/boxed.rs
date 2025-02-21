use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BoxProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,
}

/// See: https://bulma.io/documentation/elements/box/
pub fn Box(props: BoxProps) -> Element {
    rsx! {
        div {
            class: "box",
            ..props.attributes
        }
    }
}
