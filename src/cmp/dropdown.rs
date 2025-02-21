use dioxus::prelude::*;

use crate::elem::icon::FaIcon;

pub type DropdownCloseCallback = Callback<()>;
pub type DropdownRenderCallback = Callback<DropdownCloseCallback, Element>;

#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    label: Option<Element>,
    icon: Option<Element>,
    content: Callback<DropdownCloseCallback, Element>,
    #[props(default = false)]
    align_right: bool,
    #[props(default = false)]
    close_on_mouse_leave: bool,
}

pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_active = use_signal(|| false);
    let closer = use_callback(move |_: ()| {
        is_active.set(false);
    });

    rsx! {
        div {
            class: "dropdown",
            class: if props.align_right { "is-right" } else { "" },
            class: if is_active() { "is-active" } else { "" },
            div { class: "dropdown-trigger",
                button {
                    aria_haspopup: "true",
                    class: "button",
                    onclick: move |_| {
                        is_active.toggle();
                    },
                    if let Some(label) = props.label {
                        span { {label} }
                    }

                    span {
                        class: "icon is-small",
                        if let Some(icon) = props.icon {
                            {icon}
                        } else {
                            FaIcon {
                                icon: "fa-solid fa-angle-down",
                            }
                        }
                    }
                }
            }
            div {
                class: "dropdown-menu", role: "menu",
                onmouseleave: move |_| {
                    if props.close_on_mouse_leave {
                        is_active.set(false);
                    }
                },
                div { class: "dropdown-content",
                    if is_active() {
                        {
                            props.content.call(closer)
                        }
                    }
                }
            }
        }
    }
}

pub fn DropdownDivider() -> Element {
    rsx! {
        hr {
            class: "dropdown-divider"
        }
    }
}
