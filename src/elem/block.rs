use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BlockProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,
}

/// See: https://bulma.io/documentation/elements/block/
pub fn Block(props: BlockProps) -> Element {
    rsx! {
        div {
            class: "block",
            ..props.attributes
        }
    }
}
