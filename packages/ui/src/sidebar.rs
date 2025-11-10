//! Sidebar layout component

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::sidebar_nav::SidebarNav;
use crate::state::use_auth;
use dioxus::prelude::*;

/// Sidebar layout props
#[derive(Props, PartialEq, Clone)]
pub struct SidebarLayoutProps {
    pub nav_items: Option<Element>,
    pub children: Element,
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
                    class: "flex grow flex-col gap-y-5 overflow-y-auto bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 px-6 pb-4",
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
                                        class: "flex items-center gap-x-4 px-2 py-2 text-sm font-semibold leading-6 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md w-full",
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
                                            class: "flex flex-col items-start text-left",
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
                                        svg {
                                            class: "ml-auto h-5 w-5 text-gray-400",
                                            view_box: "0 0 20 20",
                                            fill: "currentColor",
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z",
                                                clip_rule: "evenodd"
                                            }
                                        }
                                    }
                                }
                                DropdownMenuContent {
                                    DropdownMenuItem::<String> {
                                        index: use_signal(|| 0usize),
                                        value: "profile".to_string(),
                                        "Profile"
                                    }
                                    DropdownMenuItem::<String> {
                                        index: use_signal(|| 1usize),
                                        value: "settings".to_string(),
                                        "Settings"
                                    }
                                    DropdownMenuItem::<String> {
                                        index: use_signal(|| 2usize),
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
                    class: "py-10",
                    {props.children}
                }
            }
        }
    }
}
