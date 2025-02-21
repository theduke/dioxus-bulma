use dioxus::prelude::*;

use crate::{Color, Size};

#[derive(Props, PartialEq, Clone)]
pub struct ProgressProps {
    #[props(extends = GlobalAttributes, extends = progress)]
    attributes: Vec<Attribute>,

    pub size: Option<Size>,
    pub color: Option<Color>,

    pub max: i32,
    pub value: Option<i32>,
}

/// See: https://bulma.io/documentation/elements/block/
pub fn Progress(props: ProgressProps) -> Element {
    let mut class = "progress".to_string();
    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(size.class());
    }
    if let Some(color) = props.color {
        class.push(' ');
        class.push_str(color.class());
    }

    rsx! {
        progress {
            class,
            max: props.max,
            value: props.value,
            ..props.attributes
        }
    }
}
