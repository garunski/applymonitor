use crate::components::login_button::LoginButton;
use crate::components::user_profile::UserProfile;
use crate::state::use_auth;
use dioxus::prelude::*;

/// Navbar component with Catalyst-inspired styling
#[component]
pub fn Navbar(children: Element) -> Element {
    let auth = use_auth();
    let user = auth.user;
    let loading = auth.loading;

    rsx! {
        nav {
            class: "bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700",
            div {
                class: "mx-auto max-w-7xl px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex h-16 justify-between items-center",
                    div {
                        class: "flex items-center",
                        div {
                            class: "flex-shrink-0",
                            h1 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white",
                                "ApplyMonitor"
                            }
                        }
                    }
                    div {
                        class: "flex items-center space-x-4",
                        {children}
                        if !loading() {
                            if user().is_some() {
                                UserProfile {}
                            } else {
                                LoginButton {}
                            }
                        }
                    }
                }
            }
        }
    }
}
