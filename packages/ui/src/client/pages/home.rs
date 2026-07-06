use dioxus::prelude::*;
use crate::client::components::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
