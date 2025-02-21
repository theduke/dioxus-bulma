//! Provides embedded bulma style sheets.
//!
//! Requires a version-specific embed feature to be enabled.
//!
//! Example:
//!
//! Cargo.toml:
//! ```toml
//! [dependencies]
//! dioxus-bulma = { version = "0.x.0", features = ["embed_v1_0_2"] }
//! ````
//!
//! ```rust
//! use dioxus::prelude::*;
//!
//! fn App() -> Element {
//!   rsx! {
//!     dioxus_bulma::embed::StylesheetBulmaV1_0_2 {}
//!   }
//! }
//! ```

use dioxus::prelude::Element;

/// Bulma v1.0.2 css stylesheet.
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn App() -> Element {
///   rsx! {
///     dioxus_bulma::embed::StylesheetBulmaV1_0_2 {}
///   }
/// }
/// ```
#[cfg(feature = "embed_v1_0_2")]
pub fn StylesheetBulmaV1_0_2() -> dioxus::prelude::Element {
    use dioxus::prelude::*;

    rsx! {
        dioxus::document::Stylesheet { href: asset!("./assets/bulma_v1.0.2.min.css") }
    }
}

#[cfg(feature = "embed_v1_0_2")]
pub fn StylesheetBulma() -> Element {
    StylesheetBulmaV1_0_2()
}
