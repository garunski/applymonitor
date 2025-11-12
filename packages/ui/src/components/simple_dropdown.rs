//! Simple dropdown menu component without primitives

use dioxus::prelude::*;

/// Simple dropdown menu props
#[derive(Props, PartialEq, Clone)]
pub struct SimpleDropdownProps {
    pub trigger: Element,
    pub children: Element,
    pub content_class: Option<String>,
}

/// Simple dropdown menu item props
#[derive(Props, PartialEq, Clone)]
pub struct SimpleDropdownItemProps {
    pub onclick: Option<EventHandler<()>>,
    pub children: Element,
}

/// Simple dropdown menu component
#[component]
pub fn SimpleDropdown(props: SimpleDropdownProps) -> Element {
    let mut is_open = use_signal(|| false);

    let base_content_class = "absolute z-50 w-56 rounded-lg bg-white p-2 shadow-lg ring-1 ring-black ring-opacity-5 dark:bg-zinc-800 dark:ring-white/10";
    let has_bottom_full = props
        .content_class
        .as_ref()
        .is_some_and(|c| c.contains("bottom-full"));
    let positioning_class = if has_bottom_full {
        "bottom-full mb-2 left-0"
    } else {
        "top-full mt-2 left-0"
    };

    let final_content_class = if let Some(custom) = props.content_class {
        format!("{} {} {}", base_content_class, positioning_class, custom)
    } else {
        format!("{} {}", base_content_class, positioning_class)
    };

    rsx! {
        div {
            class: "relative",
            div {
                onclick: move |e| {
                    e.stop_propagation();
                    is_open.set(!is_open());
                },
                {props.trigger}
            }
            if is_open() {
                div {
                    class: final_content_class,
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    {props.children}
                }
            }
        }
        if is_open() {
            div {
                class: "fixed inset-0 z-40",
                onclick: move |_| {
                    is_open.set(false);
                },
            }
        }
    }
}

/// Simple dropdown menu item component
#[component]
pub fn SimpleDropdownItem(props: SimpleDropdownItemProps) -> Element {
    rsx! {
        button {
            class: "block w-full rounded-md px-3 py-2 text-left text-sm text-zinc-900 hover:bg-zinc-50 disabled:opacity-50 disabled:cursor-not-allowed dark:text-white dark:hover:bg-zinc-700",
            onclick: move |_| {
                if let Some(handler) = &props.onclick {
                    handler.call(());
                }
            },
            {props.children}
        }
    }
}
