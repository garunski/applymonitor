use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{
    components::{account_linking::AccountLinking, timezone_settings::TimezoneSettings},
    use_auth,
};

#[component]
pub fn Accounts() -> Element {
    let auth = use_auth();
    let navigator = use_navigator();

    // Fetch user on mount
    use_effect(move || {
        auth.fetch_user();
    });

    // Redirect to login if not authenticated
    use_effect(move || {
        let user = auth.user;
        let loading = auth.loading;

        if !loading() && user().is_none() {
            navigator.push(Route::Login {});
        }
    });
    rsx! {
        document::Title { "Account Settings - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "Manage your account settings and connected providers.",
        }

        main {
            h1 {
                class: "sr-only",
                "Account Settings"
            }

            header {
                class: "border-b border-gray-200 dark:border-white/5",
                nav {
                    class: "flex overflow-x-auto py-4",
                    ul {
                        role: "list",
                        class: "flex min-w-full flex-none gap-x-6 px-4 text-sm/6 font-semibold text-gray-500 sm:px-6 lg:px-8 dark:text-gray-400",
                        li {
                            a {
                                href: "#",
                                class: "text-indigo-600 dark:text-indigo-400",
                                "Account"
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "",
                                "Notifications"
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "",
                                "Billing"
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "",
                                "Teams"
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "",
                                "Integrations"
                            }
                        }
                    }
                }
            }

            AccountLinking {}

            div {
                class: "border-t border-gray-200 dark:border-white/5",
                TimezoneSettings {}
            }
        }
    }
}
