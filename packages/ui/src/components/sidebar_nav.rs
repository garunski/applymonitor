//! Sidebar navigation components

use dioxus::prelude::*;

/// Sidebar navigation container
#[component]
pub fn SidebarNav(children: Element) -> Element {
    rsx! {
        nav {
            class: "space-y-1",
            {children}
        }
    }
}

/// Sidebar navigation item props
#[derive(Props, PartialEq, Clone)]
pub struct SidebarNavItemProps {
    pub to: Option<String>,
    pub active: Option<bool>,
    pub icon: Option<Element>,
    pub children: Element,
}

/// Sidebar navigation item
#[component]
pub fn SidebarNavItem(props: SidebarNavItemProps) -> Element {
    let active = props.active.unwrap_or(false);
    let base_class = "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold";
    let active_class = if active {
        "bg-gray-50 dark:bg-gray-800 text-brand-600 dark:text-brand-400"
    } else {
        "text-gray-700 dark:text-gray-300 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-gray-50 dark:hover:bg-gray-800"
    };

    rsx! {
        if let Some(to) = &props.to {
            a {
                href: to.clone(),
                class: "{base_class} {active_class}",
                if let Some(icon) = props.icon {
                    {icon}
                }
                {props.children}
            }
        } else {
            div {
                class: "{base_class} {active_class}",
                if let Some(icon) = props.icon {
                    {icon}
                }
                {props.children}
            }
        }
    }
}
