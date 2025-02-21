use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LevelProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub left: Option<Element>,
    pub right: Option<Element>,

    children: Element,
}

/// See: https://bulma.io/documentation/layout/level/
pub fn Level(props: LevelProps) -> Element {
    rsx! {
        div {
            class: "level",
            ..props.attributes,
            if let Some(left) = props.left {
                div {
                    class: "level-left",
                    {left}
                }
            }
            if let Some(right) = props.right {
                div {
                    class: "level-right",
                    {right}
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct LevelItemProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub text_centered: bool,

    pub children: Element,
}

/// See: https://bulma.io/documentation/layout/level/
pub fn LevelItem(props: LevelItemProps) -> Element {
    rsx! {
        div {
            class: "level-item",
            class: if props.text_centered { "has-text-centered" } else { "" },
            ..props.attributes,
            {props.children}
        }
    }
}
