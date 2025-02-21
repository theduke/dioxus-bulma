use dioxus::prelude::*;

use crate::{Color, Delete, Size};

#[derive(Props, PartialEq, Clone)]
pub struct TagProps {
    #[props(extends = GlobalAttributes, extends = span)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,
    pub color: Option<Color>,
    #[props(default = false)]
    pub hoverable: bool,
    #[props(default = false)]
    pub rounded: bool,

    /// If provided, a delete button will be added to the tag and connected
    /// to this event handler.
    pub on_delete: Option<EventHandler<Event<MouseData>>>,

    children: Element,
}

fn build_tag_class(
    size: &Option<Size>,
    color: &Option<Color>,
    hoverable: bool,
    rounded: bool,
) -> String {
    let mut class = "tag".to_string();
    if let Some(size) = size {
        class.push(' ');
        class.push_str(size.class());
    }
    if let Some(color) = color {
        class.push(' ');
        class.push_str(color.class());
    }
    if hoverable {
        class.push_str(" is-hoverable");
    }
    if rounded {
        class.push_str(" is-rounded");
    }
    class
}

/// Tag using a <span> element.
///
/// See also [`TagA`] and [`TagButton`].
///
/// See: https://bulma.io/documentation/elements/tag/
pub fn TagSpan(props: TagProps) -> Element {
    let class = build_tag_class(&props.size, &props.color, props.hoverable, props.rounded);

    rsx! {
        span {
            class,
            ..props.attributes,

            {props.children}

            {tag_deleter(&props.size, &props.on_delete)}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TagAProps {
    #[props(extends = GlobalAttributes, extends = a)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,
    pub color: Option<Color>,
    #[props(default = false)]
    pub hoverable: bool,
    #[props(default = false)]
    pub rounded: bool,

    pub href: String,

    pub on_delete: Option<EventHandler<Event<MouseData>>>,

    children: Element,
}

/// Tag using an <a> element.
/// See also [`Tag`] and [`TagButton`].
///
/// See: https://bulma.io/documentation/elements/tag/
pub fn TagA(props: TagAProps) -> Element {
    let class = build_tag_class(&props.size, &props.color, props.hoverable, props.rounded);

    rsx! {
        a {
            class,
            href: props.href,
            ..props.attributes,

            {props.children}

            {tag_deleter(&props.size, &props.on_delete)}
        }
    }
}

fn tag_deleter(size: &Option<Size>, handler: &Option<EventHandler<Event<MouseData>>>) -> Element {
    rsx! {
        if let Some(handler) = handler {
            Delete {
                size: match size {
                    Some(Size::Large) => None,
                    _ => Some(Size::Small),
                },
                onclick: *handler,
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TagButtonProps {
    #[props(extends = GlobalAttributes, extends = button)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,
    pub color: Option<Color>,
    #[props(default = false)]
    pub hoverable: bool,
    #[props(default = false)]
    pub rounded: bool,

    pub on_click: EventHandler<Event<MouseData>>,
    pub on_delete: Option<EventHandler<Event<MouseData>>>,

    children: Element,
}

/// Tag using a <button> element.
/// See also [`Tag`] and [`TagA`].
///
/// See: https://bulma.io/documentation/elements/tag/
pub fn TagButton(props: TagButtonProps) -> Element {
    let class = build_tag_class(&props.size, &props.color, props.hoverable, props.rounded);

    rsx! {
        button {
            class,
            onclick: props.on_click,
            ..props.attributes,

            {props.children}

            {tag_deleter(&props.size, &props.on_delete)}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TagsProps {
    #[props(extends = GlobalAttributes, extends = div)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,

    #[props(default = false)]
    pub has_addons: bool,

    children: Element,
}

/// See: https://bulma.io/documentation/elements/tag/
pub fn Tags(props: TagsProps) -> Element {
    let mut class = "tags".to_string();
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class_multiple());
    }
    if props.has_addons {
        class.push_str(" has-addons");
    }

    rsx! {
        div {
            class,
            ..props.attributes,
            {props.children}
        }
    }
}
