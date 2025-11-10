use crate::components::button::{Button, ButtonVariant};
use crate::state::auth_state::use_auth;
use dioxus::prelude::*;

/// Account linking component
#[component]
pub fn AccountLinking() -> Element {
    let auth = use_auth();
    let user = auth.user;

    let providers_opt = user().map(|u| u.providers.clone());
    let providers_count = providers_opt.as_ref().map(|p| p.len()).unwrap_or(0);

    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-2xl font-bold text-gray-900 dark:text-white",
                "Connected Accounts"
            }
            if let Some(providers) = providers_opt {
                div {
                    class: "space-y-4",
                    // Display linked providers
                    for provider in providers.clone() {
                        div {
                            class: "flex items-center justify-between p-4 border border-gray-300 dark:border-gray-600 rounded-lg",
                            div {
                                class: "flex items-center gap-3",
                                span {
                                    class: "font-medium text-gray-900 dark:text-white",
                                    {provider.to_uppercase()}
                                }
                                if provider == "local" {
                                    span {
                                        class: "text-sm text-gray-500 dark:text-gray-400",
                                        "Email/Password"
                                    }
                                }
                            }
                            if providers_count > 1 {
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    class: "text-red-600 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300",
                                    onclick: move |_| {
                                        auth.unlink_provider(provider.clone());
                                    },
                                    "Unlink"
                                }
                            } else {
                                span {
                                    class: "text-sm text-gray-500 dark:text-gray-400",
                                    "Cannot unlink last provider"
                                }
                            }
                        }
                    }

                    // Available providers to link
                    div {
                        class: "mt-6 pt-6 border-t border-gray-300 dark:border-gray-600",
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                            "Link Additional Account"
                        }
                        div {
                            class: "space-y-3",
                            if !providers.contains(&"google".to_string()) {
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    class: "w-full flex items-center justify-center gap-3 text-base px-6 py-3 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700",
                                    onclick: move |_| {
                                        auth.link_provider("google".to_string());
                                    },
                                    svg {
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            fill: "currentColor",
                                            d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                                        }
                                        path {
                                            fill: "currentColor",
                                            d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                                        }
                                        path {
                                            fill: "currentColor",
                                            d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                                        }
                                        path {
                                            fill: "currentColor",
                                            d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                                        }
                                    }
                                    "Link Google Account"
                                }
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "text-center text-gray-500 dark:text-gray-400",
                    "Please sign in to view connected accounts"
                }
            }
        }
    }
}
