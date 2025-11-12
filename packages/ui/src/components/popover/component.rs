use dioxus::prelude::*;
use dioxus_primitives::popover::{
    self, PopoverContentProps, PopoverRootProps, PopoverTriggerProps,
};

#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    rsx! {
        popover::PopoverRoot {
            class: "relative",
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        popover::PopoverTrigger {
            class: "",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        popover::PopoverContent {
            class: "absolute z-10 mt-2 w-64 rounded-lg bg-white p-2 shadow-lg ring-1 ring-black ring-opacity-5 dark:bg-zinc-800 dark:ring-white/10",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
