use dioxus::prelude::*;

use ui::{use_auth_provider, SidebarLayout};
use views::{Accounts, Blog, Dashboard, Home, Jobs, Login};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[layout(WebSidebar)]
    #[route("/dashboard")]
    Dashboard {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/jobs")]
    Jobs {},
    #[route("/settings/accounts")]
    Accounts {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize auth state provider
    let auth = use_auth_provider();

    // Check for OIDC callback on mount
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().expect("no global `window` exists");
            let location = window.location();
            let href = location.href().unwrap_or_default();

            // Check if we're coming back from OIDC callback
            // The callback is handled server-side, so we just need to fetch user
            if !href.contains("error") {
                auth.fetch_user();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = auth;
        }
    });

    rsx! {
        // Global app resources
        document::Stylesheet { href: TAILWIND_CSS }

        // Default metadata
        document::Title { "ApplyMonitor - Track Your Job Applications" }
        document::Meta {
            name: "description",
            content: "Track and manage your job applications effortlessly. Automatically organize applications, interviews, and follow-ups all in one place.",
        }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        document::Meta {
            property: "og:title",
            content: "ApplyMonitor - Track Your Job Applications",
        }
        document::Meta {
            property: "og:description",
            content: "Track and manage your job applications effortlessly. Automatically organize applications, interviews, and follow-ups all in one place.",
        }
        document::Meta {
            property: "og:type",
            content: "website",
        }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `SidebarLayout` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebSidebar() -> Element {
    use ui::components::sidebar_nav::SidebarNav;

    rsx! {
        SidebarLayout {
            nav_items: Some(rsx! {
                SidebarNav {
                    Link {
                            to: Route::Dashboard {},
                            class: "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-gray-50 dark:hover:bg-gray-800",
                            svg {
                                class: "h-6 w-6 shrink-0",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                                }
                            }
                            "Dashboard"
                        }
                        Link {
                            to: Route::Jobs {},
                            class: "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-gray-50 dark:hover:bg-gray-800",
                            svg {
                                class: "h-6 w-6 shrink-0",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M20.25 14.15v4.25c0 .414-.336.75-.75.75h-4.5a.75.75 0 01-.75-.75v-4.25m0 0h4.5v-4.5H15m-1.5 4.5v-4.5m0 0h-4.5m4.5 0V9.75m0 0h-4.5m4.5 0v-4.5m0 0h-4.5"
                                }
                            }
                            "Jobs"
                        }
                        Link {
                            to: Route::Accounts {},
                            class: "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-gray-50 dark:hover:bg-gray-800",
                            svg {
                                class: "h-6 w-6 shrink-0",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
                                }
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                }
                            }
                            "Settings"
                        }
                }
            }),
            Outlet::<Route> {}
        }
    }
}
