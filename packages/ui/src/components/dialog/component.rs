use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogContentProps, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    rsx! {
        dialog::DialogRoot {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/30 dark:bg-black/50",
            id: props.id,
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
pub fn DialogContent(props: DialogContentProps) -> Element {
    rsx! {
        dialog::DialogContent {
            class: "relative z-50 w-full max-w-lg rounded-lg bg-white p-6 shadow-xl dark:bg-zinc-800",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    rsx! {
        dialog::DialogTitle {
            class: "text-base/7 font-semibold text-zinc-900 dark:text-white",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        dialog::DialogDescription {
            class: "mt-2 text-sm/6 text-zinc-600 dark:text-zinc-400",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
