//! Sidebar layout component

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::sidebar_nav::SidebarNav;
use crate::state::use_auth;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsChevronUp, Icon};

/// Sidebar layout props
#[derive(Props, PartialEq, Clone)]
pub struct SidebarLayoutProps {
    pub nav_items: Option<Element>,
    pub children: Element,
    pub on_settings_click: Option<EventHandler<()>>,
}

/// Sidebar layout component with fixed sidebar and main content area
#[component]
pub fn SidebarLayout(props: SidebarLayoutProps) -> Element {
    let auth = use_auth();
    let user = auth.user;

    rsx! {
        div {
            class: "flex h-screen bg-gray-50 dark:bg-gray-950",
            // Sidebar
            div {
                class: "hidden lg:flex lg:flex-col lg:w-72 lg:fixed lg:inset-y-0 lg:z-50",
                div {
                    class: "flex grow flex-col gap-y-5 overflow-y-auto bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 px-6 py-6",
                    // Top section: Logo
                    div {
                        class: "flex h-16 shrink-0 items-center",
                        h1 {
                            class: "text-xl font-semibold text-gray-900 dark:text-white",
                            "ApplyMonitor"
                        }
                    }

                    // Navigation
                    if let Some(nav_items) = props.nav_items {
                        SidebarNav {
                            {nav_items}
                        }
                    }

                    // Bottom section: User profile
                    div {
                        class: "mt-auto",
                        if let Some(user) = user() {
                            DropdownMenu {
                                DropdownMenuTrigger {
                                    button {
                                        class: "flex items-center gap-x-3 px-2 py-2 text-sm font-semibold leading-6 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md w-full",
                                        if let Some(ref picture) = user.picture {
                                            img {
                                                src: picture.clone(),
                                                alt: user.name.as_deref().unwrap_or("User"),
                                                class: "h-8 w-8 rounded-full bg-gray-50 dark:bg-gray-800"
                                            }
                                        } else {
                                            div {
                                                class: "h-8 w-8 rounded-full bg-blue-600 dark:bg-blue-500 flex items-center justify-center text-white text-sm font-medium",
                                                {user.name.as_ref().and_then(|n| n.chars().next()).unwrap_or('U').to_uppercase().collect::<String>()}
                                            }
                                        }
                                        div {
                                            class: "flex flex-col items-start text-left flex-1 min-w-0 gap-y-0.5",
                                            span {
                                                class: "text-sm font-semibold text-gray-900 dark:text-white truncate",
                                                {user.name.as_deref().unwrap_or_else(|| user.email.as_deref().unwrap_or("User"))}
                                            }
                                            if let Some(ref email) = user.email {
                                                span {
                                                    class: "text-xs text-gray-500 dark:text-gray-400 truncate",
                                                    {email.clone()}
                                                }
                                            }
                                        }
                                        Icon {
                                            class: "h-5 w-5 text-gray-400 shrink-0 ml-auto",
                                            width: 20,
                                            height: 20,
                                            fill: "currentColor",
                                            icon: BsChevronUp,
                                        }
                                    }
                                }
                                DropdownMenuContent {
                                    class: "right-0 w-64",
                                    // User info section
                                    div {
                                        class: "px-4 py-3 border-b border-gray-200 dark:border-gray-700",
                                        div {
                                            class: "flex items-center gap-3",
                                            if let Some(ref picture) = user.picture {
                                                img {
                                                    src: picture.clone(),
                                                    alt: user.name.as_deref().unwrap_or("User"),
                                                    class: "h-10 w-10 rounded-full bg-gray-50 dark:bg-gray-800"
                                                }
                                            } else {
                                                div {
                                                    class: "h-10 w-10 rounded-full bg-blue-600 dark:bg-blue-500 flex items-center justify-center text-white text-sm font-medium",
                                                    {user.name.as_ref().and_then(|n| n.chars().next()).unwrap_or('U').to_uppercase().collect::<String>()}
                                                }
                                            }
                                            div {
                                                class: "flex flex-col",
                                                span {
                                                    class: "text-sm font-semibold text-gray-900 dark:text-white",
                                                    {user.name.as_deref().unwrap_or_else(|| user.email.as_deref().unwrap_or("User"))}
                                                }
                                                if let Some(ref email) = user.email {
                                                    span {
                                                        class: "text-xs text-gray-500 dark:text-gray-400",
                                                        {email.clone()}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    DropdownMenuItem::<String> {
                                        index: use_signal(|| 0usize),
                                        value: "settings".to_string(),
                                        on_select: move |_| {
                                            if let Some(handler) = &props.on_settings_click {
                                                handler.call(());
                                            }
                                        },
                                        "Settings"
                                    }
                                    DropdownMenuItem::<String> {
                                        index: use_signal(|| 1usize),
                                        value: "signout".to_string(),
                                        on_select: move |_| {
                                            auth.logout();
                                        },
                                        "Sign out"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Main content area
            div {
                class: "lg:pl-72 flex flex-col flex-1",
                main {
                    class: "py-6",
                    {props.children}
                }
            }
        }
    }
}
