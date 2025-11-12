use dioxus::prelude::*;
use dioxus_primitives::separator::{self, SeparatorProps};

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let separator_class = if props.horizontal {
        "h-px w-full bg-zinc-200 dark:bg-white/10"
    } else {
        "h-full w-px bg-zinc-200 dark:bg-white/10"
    };
    rsx! {
        separator::Separator {
            class: separator_class,
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: props.attributes,
            {props.children}
        }
    }
}
