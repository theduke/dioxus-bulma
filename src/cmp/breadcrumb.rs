use dioxus::prelude::*;

use crate::Size;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BreadcrumbAlign {
    Left,
    Center,
    Right,
}

impl BreadcrumbAlign {
    pub fn class(&self) -> &str {
        match self {
            Self::Left => "",
            Self::Center => "is-center",
            Self::Right => "is-right",
        }
    }
}

impl std::fmt::Display for BreadcrumbAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BreadcrumbSeparator {
    Arrow,
    Bullet,
    Dot,
    Succeeds,
}

impl BreadcrumbSeparator {
    pub fn class(&self) -> &str {
        match self {
            Self::Arrow => "has-arrow-separator",
            Self::Bullet => "has-bullet-separator",
            Self::Dot => "has-dot-separator",
            Self::Succeeds => "has-succeeds-separator",
        }
    }
}

impl std::fmt::Display for BreadcrumbSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class())
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbProps {
    #[props(extends = GlobalAttributes, extends = nav)]
    attributes: Vec<Attribute>,

    pub align: Option<BreadcrumbAlign>,
    pub separator: Option<BreadcrumbSeparator>,
    pub size: Option<Size>,

    children: Element,
}

pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let mut class = "breadcrumb".to_string();
    if let Some(align) = props.align {
        class.push_str(align.class());
    }
    if let Some(separator) = props.separator {
        class.push_str(separator.class());
    }
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }

    rsx! {
        nav {
            class: "breadcrumb",
            ..props.attributes,

            ul {
                {props.children}
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbItemProps {
    #[props(extends = GlobalAttributes, extends = li)]
    attributes: Vec<Attribute>,

    #[props(default = false)]
    pub active: bool,

    children: Element,
}

pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    rsx! {
        li {
            class: if props.active { "is-active" } else { "" },
            ..props.attributes,

            {props.children}
        }
    }
}
