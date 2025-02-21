use dioxus::prelude::*;

use crate::{Color, Size};

#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    color: Option<Color>,
    size: Option<Size>,
    text: Option<String>,
    children: Element,
}

/// Wrapper for icons with associated text.
/// Can contain multiple icon + text pairs.
///
/// See: https://bulma.io/documentation/elements/icon/
#[component]
pub fn IconText(children: Element) -> Element {
    rsx! {
        span {
            class: "icon-text",
            {children}
        }
    }
}

/// See: https://bulma.io/documentation/elements/icon/
pub fn Icon(props: IconProps) -> Element {
    let mut cls = "icon".to_string();
    if let Some(color) = props.color {
        cls.push(' ');
        cls.push_str(color.text_class());
    }
    if let Some(size) = props.size {
        cls.push(' ');
        cls.push_str(size.class());
    }

    if let Some(text) = props.text {
        rsx! {
            span {
                class: "icon-text",
                span {
                    class: "icon",
                    {props.children}
                }
                span {
                    {text}
                }
            }
        }
    } else {
        rsx! {
            span {
                class: "icon",
                {props.children}
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FaIconProps {
    color: Option<Color>,
    size: Option<Size>,
    text: Option<String>,
    pub icon: &'static str,
}

/// Font Awesome icon.
///
/// See: https://bulma.io/documentation/elements/icon/
pub fn FaIcon(props: FaIconProps) -> Element {
    rsx! {
        Icon {
            color: props.color,
            size: props.size,
            text: props.text,

            i {
                class: props.icon,
                aria_hidden: "true",
            }
        }
    }
}
