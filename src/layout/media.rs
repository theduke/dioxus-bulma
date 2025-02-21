use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MediaProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub left: Option<Element>,
    pub right: Option<Element>,
    pub content: Element,
}

/// See: https://bulma.io/documentation/layout/media/
pub fn Media(props: MediaProps) -> Element {
    rsx! {
        article {
            class: "media",
            ..props.attributes,
            if let Some(left) = props.left {
                figure {
                    class: "media-left",
                    {left}
                }
            }
            if let Some(right) = props.right {
                div {
                    class: "media-right",
                    {right}
                }
            }
            div {
                class: "media-content",
                {props.content}
            }
        }
    }
}
