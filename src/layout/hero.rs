use dioxus::prelude::*;

use crate::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeroSize {
    Small,
    Medium,
    Large,
    HalfHeight,
    FullHeight,
    FullHeightWithNavbar,
}

impl HeroSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Small => "is-small",
            Self::Medium => "is-medium",
            Self::Large => "is-large",
            Self::HalfHeight => "is-halfheight",
            Self::FullHeight => "is-fullheight",
            Self::FullHeightWithNavbar => "is-fullheight-with-navbar",
        }
    }
}

impl std::fmt::Display for HeroSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.class())
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct HeroProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    size: Option<HeroSize>,
    color: Option<Color>,

    head: Option<Element>,
    foot: Option<Element>,

    children: Element,
}

/// See: https://bulma.io/documentation/layout/hero/
pub fn Hero(props: HeroProps) -> Element {
    let mut class = "her".to_string();
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }
    if let Some(color) = props.color {
        class.push(' ');
        class.push_str(color.class());
    }
    rsx! {
        div {
            class,
            ..props.attributes,

            if let Some(head) = props.head {
                div {
                    class: "hero-head",
                    {head}
                }
            }

            div {
                class: "hero-body",
                {props.children}
            }

            if let Some(foot) = props.foot {
                div {
                    class: "hero-foot",
                    {foot}
                }
            }
        }
    }
}
