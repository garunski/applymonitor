use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsBriefcase, BsGear, BsHouse},
    Icon,
};

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

    // Fetch user on mount
    use_effect(move || {
        auth.fetch_user();
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
    use dioxus_router::use_navigator;
    use ui::components::sidebar_nav::SidebarNav;

    let navigator = use_navigator();

    rsx! {
        SidebarLayout {
            on_settings_click: move |_| {
                navigator.push(Route::Accounts {});
            },
            nav_items: Some(rsx! {
                SidebarNav {
                    Link {
                            to: Route::Dashboard {},
                            class: "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-gray-300 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-gray-50 dark:hover:bg-gray-800",
                            Icon {
                                class: "h-6 w-6 shrink-0",
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsHouse,
                            }
                            "Dashboard"
                        }
                        Link {
                            to: Route::Jobs {},
                            class: "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-gray-300 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-gray-50 dark:hover:bg-gray-800",
                            Icon {
                                class: "h-6 w-6 shrink-0",
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsBriefcase,
                            }
                            "Jobs"
                        }
                        Link {
                            to: Route::Accounts {},
                            class: "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-gray-300 hover:text-brand-600 dark:hover:text-brand-400 hover:bg-gray-50 dark:hover:bg-gray-800",
                            Icon {
                                class: "h-6 w-6 shrink-0",
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsGear,
                            }
                            "Settings"
                        }
                }
            }),
            Outlet::<Route> {}
        }
    }
}
