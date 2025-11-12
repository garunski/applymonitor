use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsChevronDown, Icon};
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        navbar::Navbar {
            class: "flex gap-x-1 rounded-lg bg-white/90 px-1 py-1 text-sm/6 font-semibold text-zinc-700 shadow-lg shadow-zinc-900/5 ring-1 ring-zinc-900/10 dark:bg-zinc-800 dark:text-zinc-200 dark:ring-white/10",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            class: "relative",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger {
            class: "group relative rounded-md px-3 py-2 hover:bg-zinc-50 dark:hover:bg-zinc-700 flex items-center gap-x-1",
            attributes: props.attributes,
            {props.children}
            Icon {
                class: "h-5 w-5 shrink-0 text-zinc-400 dark:text-zinc-500 transition-transform group-data-[state=open]:rotate-180",
                width: 20,
                height: 20,
                fill: "currentColor",
                icon: BsChevronDown,
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            class: "absolute z-10 mt-2 min-w-[200px] rounded-lg bg-white p-2 shadow-lg ring-1 ring-black ring-opacity-5 dark:bg-zinc-800 dark:ring-white/10",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        navbar::NavbarItem {
            class: "group relative rounded-md px-3 py-2 text-sm text-zinc-900 hover:bg-zinc-50 disabled:opacity-50 disabled:cursor-not-allowed dark:text-white dark:hover:bg-zinc-700",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}
