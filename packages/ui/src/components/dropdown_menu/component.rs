use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenuItemProps, DropdownMenuProps, DropdownMenuTriggerProps,
};

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    rsx! {
        dropdown_menu::DropdownMenu {
            class: "relative",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    rsx! {
        dropdown_menu::DropdownMenuTrigger {
            class: "",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuContent(
    class: Option<String>,
    id: Option<String>,
    #[props(default)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    // Check if opening upward (bottom-full)
    let has_bottom_full = class.as_ref().is_some_and(|c| c.contains("bottom-full"));

    // Base classes - remove mt-2 if opening upward, remove max-h/overflow to prevent scrollbar
    let base_class = if has_bottom_full {
        "absolute z-10 w-64 rounded-lg bg-white p-2 shadow-lg ring-1 ring-black ring-opacity-5 dark:bg-zinc-800 dark:ring-white/10"
    } else {
        "absolute z-10 mt-2 w-64 rounded-lg bg-white p-2 shadow-lg ring-1 ring-black ring-opacity-5 dark:bg-zinc-800 dark:ring-white/10"
    };

    // Merge base class with custom class
    let final_class = if let Some(custom) = class {
        format!("{} {}", base_class, custom)
    } else {
        base_class.to_string()
    };

    rsx! {
        dropdown_menu::DropdownMenuContent {
            class: final_class,
            id: id,
            attributes: attributes,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    rsx! {
        dropdown_menu::DropdownMenuItem {
            class: "block w-full rounded-md px-3 py-2 text-left text-sm text-zinc-900 hover:bg-zinc-50 disabled:opacity-50 disabled:cursor-not-allowed dark:text-white dark:hover:bg-zinc-700",
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
