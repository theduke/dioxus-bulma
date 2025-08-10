use dioxus::prelude::*;

#[component]
pub fn Modal(
    children: Element,
    on_close: EventHandler<()>,
    content_class: Option<&'static str>,
) -> Element {
    // bulma modal
    rsx! {
        div {
            class: "modal is-active",
            div {
                class: "modal-background",
                onclick: move |_| {
                    on_close.call(());
                }
            }
            div {
                class: "modal-content",
                class: if let Some(class) = content_class {
                    "{class}"
                } else {
                    ""
                },
                {children}
            }
            button {
                class: "modal-close is-large",
                aria_label: "close",
                onclick: move |_| {
                    on_close.call(());
                }
            }
        }
    }
}
