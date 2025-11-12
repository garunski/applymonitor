use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            class: "block text-sm/6 font-semibold text-zinc-900 dark:text-white",
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
