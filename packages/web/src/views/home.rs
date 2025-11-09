use dioxus::prelude::*;
use ui::{Hero, ApiTest};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        ApiTest {}
    }
}
