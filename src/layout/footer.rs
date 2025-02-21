use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FooterProps {
    #[props(extends = GlobalAttributes, extends = footer)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// See: https://bulma.io/documentation/layout/footer/
pub fn Footer(props: FooterProps) -> Element {
    rsx! {
        footer {
            class: "footer",
            ..props.attributes,
            {props.children}
        }
    }
}
