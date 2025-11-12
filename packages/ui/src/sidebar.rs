//! Sidebar layout component

use crate::components::sidebar_nav::SidebarNav;
use crate::components::simple_dropdown::{SimpleDropdown, SimpleDropdownItem};
use crate::state::use_auth;
use dioxus::prelude::*;

const LOGO_SVG: Asset = asset!("/assets/logo.svg");

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
                class: "hidden lg:flex lg:flex-col lg:w-64 lg:fixed lg:inset-y-0 lg:z-50 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800",
                div {
                    class: "flex grow flex-col min-h-0",
                    // Scrollable content area
                    div {
                        class: "flex-1 overflow-y-auto px-6 py-6",
                        div {
                            class: "flex flex-col gap-y-5",
                            // Top section: Logo
                            div {
                                class: "flex h-16 shrink-0 items-center gap-3",
                                img {
                                    src: LOGO_SVG,
                                    alt: "ApplyMonitor",
                                    class: "h-10 w-10",
                                }
                                h1 {
                                    class: "text-xl font-semibold",
                                    span {
                                        class: "text-brand-500",
                                        "Apply"
                                    }
                                    " "
                                    span {
                                        class: "text-brand-900 dark:text-white",
                                        "Monitor"
                                    }
                                }
                            }

                            // Navigation
                            if let Some(nav_items) = props.nav_items {
                                SidebarNav {
                                    {nav_items}
                                }
                            }
                        }
                    }

                    // Bottom section: User profile (outside scroll container)
                    div {
                        class: "shrink-0 border-t border-zinc-950/5 dark:border-white/5 px-6 pt-4 pb-6",
                        if let Some(user) = user() {
                            SimpleDropdown {
                                content_class: Some("bottom-full mb-2 left-0".to_string()),
                                trigger: rsx! {
                                    button {
                                        class: "flex w-full items-center gap-3 rounded-lg px-2 py-2.5 text-left text-base/6 font-medium text-zinc-950 hover:bg-zinc-950/5 dark:text-white dark:hover:bg-white/5 sm:py-2 sm:text-sm/5",
                                        if let Some(ref picture) = user.picture {
                                            img {
                                                src: picture.clone(),
                                                alt: user.name.as_deref().unwrap_or("User"),
                                                class: "h-7 w-7 shrink-0 rounded-full sm:h-6 sm:w-6"
                                            }
                                        } else {
                                            div {
                                                class: "h-7 w-7 shrink-0 rounded-full bg-brand-600 dark:bg-brand-500 flex items-center justify-center text-white text-sm font-medium sm:h-6 sm:w-6",
                                                {user.name.as_ref().and_then(|n| n.chars().next()).unwrap_or('U').to_uppercase().collect::<String>()}
                                            }
                                        }
                                        div {
                                            class: "flex flex-col items-start text-left flex-1 min-w-0 gap-y-0.5",
                                            span {
                                                class: "text-sm font-semibold text-zinc-950 dark:text-white truncate sm:text-xs",
                                                {user.name.as_deref().unwrap_or_else(|| user.email.as_deref().unwrap_or("User"))}
                                            }
                                            if let Some(ref email) = user.email {
                                                span {
                                                    class: "text-xs text-zinc-500 dark:text-zinc-400 truncate",
                                                    {email.clone()}
                                                }
                                            }
                                        }
                                    }
                                },
                                SimpleDropdownItem {
                                    onclick: move |_| {
                                        if let Some(handler) = &props.on_settings_click {
                                            handler.call(());
                                        }
                                    },
                                    "Settings"
                                }
                                SimpleDropdownItem {
                                    onclick: move |_| {
                                        auth.logout();
                                    },
                                    "Sign out"
                                }
                            }
                        }
                    }
                }
            }

            // Main content area
            div {
                class: "lg:pl-64 flex flex-col flex-1",
                main {
                    class: "py-6",
                    {props.children}
                }
            }
        }
    }
}
