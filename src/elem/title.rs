use dioxus::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TitleSize {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl TitleSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::H1 => "is-1",
            Self::H2 => "is-2",
            Self::H3 => "is-3",
            Self::H4 => "is-4",
            Self::H5 => "is-5",
            Self::H6 => "is-6",
        }
    }

    /// Subtitles use a size two steps smaller than the title.
    fn subtitle_size(self) -> Self {
        match self {
            Self::H1 => Self::H3,
            Self::H2 => Self::H4,
            Self::H3 => Self::H5,
            Self::H4 | Self::H5 | Self::H6 => Self::H6,
        }
    }
}

impl std::fmt::Display for TitleSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.class())
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TitleProps {
    #[props(extends = GlobalAttributes, extends = h1)]
    attributes: Vec<Attribute>,

    size: TitleSize,

    subtitle: Option<Element>,

    /// Use a <p> tag instead of an <h1/h2/...> tag.
    #[props(default = false)]
    paragraph: bool,

    /// Add the is-spaced modifier to the title.
    #[props(default = false)]
    spaced: bool,

    children: Element,
}

/// See: https://bulma.io/documentation/elements/block/
pub fn Title(props: TitleProps) -> Element {
    let class = props.size.class();

    rsx! {
        p {
            class,
            ..props.attributes,

            {props.children}
        }

        if let Some(subtitle) = props.subtitle {
            p {
                class: props.size.subtitle_size().class(),

                {subtitle}
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SubtitleProps {
    #[props(extends = GlobalAttributes, extends = h1)]
    attributes: Vec<Attribute>,

    size: TitleSize,

    children: Element,
}

pub fn Subtitle(props: TitleProps) -> Element {
    let class = props.size.subtitle_size().class();

    rsx! {
        p {
            class,
            ..props.attributes,

            {props.children}
        }
    }
}
