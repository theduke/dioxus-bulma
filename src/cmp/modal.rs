use dioxus::prelude::*;

#[component]
pub fn Modal(children: Element, on_close: EventHandler<()>) -> Element {
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
