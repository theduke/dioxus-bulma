use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContainerSize {
    Widescreen,
    FullHd,
    MaxDesktop,
    MaxWidescreen,
    MaxTablet,
    Fluid,
}

impl ContainerSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Widescreen => "is-widescreen",
            Self::FullHd => "is-fullhd",
            Self::MaxDesktop => "is-max-desktop",
            Self::MaxWidescreen => "is-max-widescreen",
            Self::MaxTablet => "is-max-tablet",
            Self::Fluid => "is-fluid",
        }
    }
}

impl std::fmt::Display for ContainerSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.class())
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ContainerProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    size: Option<ContainerSize>,

    children: Element,
}

/// See: https://bulma.io/documentation/layout/container/
pub fn Container(props: ContainerProps) -> Element {
    let mut class = "container".to_string();
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
