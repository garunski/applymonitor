use crate::components::button::{Button, ButtonVariant};
use crate::services::gmail_scanner_service::{GmailScannerService, GmailStatus};
use crate::state::auth_state::use_auth;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsCheckCircle, Icon};

/// Available auth providers
const AVAILABLE_PROVIDERS: &[(&str, &str)] = &[("Email/Password", "local"), ("Google", "google")];

/// Account linking component
#[component]
pub fn AccountLinking() -> Element {
    let auth = use_auth();
    let user = auth.user;

    let providers_opt = user().map(|u| u.providers.clone());
    let providers_count = providers_opt.as_ref().map(|p| p.len()).unwrap_or(0);
    let gmail_status = use_signal(|| None::<GmailStatus>);

    // Fetch Gmail connection status on mount
    use_effect(move || {
        let mut status = gmail_status;
        spawn(async move {
            match GmailScannerService::get_scan_status().await {
                Ok(status_data) => {
                    *status.write() = Some(status_data);
                }
                Err(_) => {
                    *status.write() = None;
                }
            }
        });
    });

    let gmail_connected = gmail_status
        .read()
        .as_ref()
        .map(|s| s.connected)
        .unwrap_or(false);

    rsx! {
        div {
            class: "divide-y divide-gray-200 dark:divide-white/10",
            // Authentication Providers Section
            div {
                class: "grid max-w-7xl grid-cols-1 gap-x-8 gap-y-10 px-4 py-16 sm:px-6 md:grid-cols-3 lg:px-8",
                div {
                    h2 {
                        class: "text-base/7 font-semibold text-gray-900 dark:text-white",
                        "Authentication Providers"
                    }
                    p {
                        class: "mt-1 text-sm/6 text-gray-500 dark:text-gray-400",
                        "Manage how you sign in to your account. You can link multiple providers for convenience."
                    }
                }

                div {
                    class: "md:col-span-2",
                    div {
                        class: "space-y-4",
                        if let Some(providers) = providers_opt {
                            for (provider_name, provider_key) in AVAILABLE_PROVIDERS.iter() {
                                div {
                                    class: "flex items-center justify-between rounded-lg border border-gray-200 dark:border-white/10 bg-white dark:bg-white/5 p-4",
                                    div {
                                        class: "flex items-center gap-4",
                                        if *provider_key == "google" {
                                            svg {
                                                class: "size-6 shrink-0",
                                                view_box: "0 0 24 24",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                path {
                                                    fill: "#4285F4",
                                                    d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                                                }
                                                path {
                                                    fill: "#34A853",
                                                    d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                                                }
                                                path {
                                                    fill: "#FBBC05",
                                                    d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                                                }
                                                path {
                                                    fill: "#EA4335",
                                                    d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                                                }
                                            }
                                        } else {
                                            svg {
                                                class: "size-6 shrink-0 text-gray-400 dark:text-gray-500",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "1.5",
                                                path {
                                                    d: "M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75",
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                }
                                            }
                                        }
                                        div {
                                            div {
                                                class: "text-sm/6 font-semibold text-gray-900 dark:text-white",
                                                {*provider_name}
                                            }
                                            if providers.contains(&provider_key.to_string()) {
                                                div {
                                                    class: "mt-1 flex items-center gap-1.5 text-xs/5 text-green-600 dark:text-green-400",
                                                    Icon {
                                                        class: "size-4",
                                                        width: 16,
                                                        height: 16,
                                                        fill: "currentColor",
                                                        icon: BsCheckCircle,
                                                    }
                                                    "Connected"
                                                }
                                            } else {
                                                div {
                                                    class: "mt-1 text-xs/5 text-gray-500 dark:text-gray-400",
                                                    "Not connected"
                                                }
                                            }
                                        }
                                    }
                                    if providers.contains(&provider_key.to_string()) {
                                        if providers_count > 1 {
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                class: "text-sm font-semibold text-red-600 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300",
                                                onclick: move |_| {
                                                    auth.unlink_provider(provider_key.to_string());
                                                },
                                                "Unlink"
                                            }
                                        } else {
                                            span {
                                                class: "text-sm text-gray-500 dark:text-gray-400",
                                                "Cannot unlink last provider"
                                            }
                                        }
                                    } else {
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "text-sm font-semibold text-indigo-600 hover:text-indigo-700 dark:text-indigo-400 dark:hover:text-indigo-300",
                                            onclick: move |_| {
                                                auth.link_provider(provider_key.to_string());
                                            },
                                            "Connect"
                                        }
                                    }
                                }
                            }
                        } else {
                            div {
                                class: "text-center text-gray-500 dark:text-gray-400 py-8",
                                "Please sign in to view connected accounts"
                            }
                        }
                    }
                }
            }

            // Gmail Integration Section
            div {
                class: "grid max-w-7xl grid-cols-1 gap-x-8 gap-y-10 px-4 py-16 sm:px-6 md:grid-cols-3 lg:px-8",
                div {
                    h2 {
                        class: "text-base/7 font-semibold text-gray-900 dark:text-white",
                        "Gmail Integration"
                    }
                    p {
                        class: "mt-1 text-sm/6 text-gray-500 dark:text-gray-400",
                        "Connect your Gmail account to automatically scan for job application emails."
                    }
                }

                div {
                    class: "md:col-span-2",
                    div {
                        class: "rounded-lg border border-gray-200 dark:border-white/10 bg-white dark:bg-white/5 p-4",
                        div {
                            class: "flex items-center justify-between",
                            div {
                                class: "flex items-center gap-4",
                                svg {
                                    class: "size-6 shrink-0 text-gray-400 dark:text-gray-500",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "1.5",
                                    path {
                                        d: "M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                    }
                                }
                                div {
                                    div {
                                        class: "text-sm/6 font-semibold text-gray-900 dark:text-white",
                                        "Gmail"
                                    }
                                    if gmail_connected {
                                        div {
                                            class: "mt-1 flex items-center gap-1.5 text-xs/5 text-green-600 dark:text-green-400",
                                            Icon {
                                                class: "size-4",
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: BsCheckCircle,
                                            }
                                            "Connected"
                                        }
                                    } else {
                                        div {
                                            class: "mt-1 text-xs/5 text-gray-500 dark:text-gray-400",
                                            "Not connected"
                                        }
                                    }
                                }
                            }
                            if !gmail_connected {
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    class: "text-sm font-semibold text-indigo-600 hover:text-indigo-700 dark:text-indigo-400 dark:hover:text-indigo-300",
                                    onclick: move |_| {
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            let auth_url = GmailScannerService::get_gmail_auth_url();
                                            let window = web_sys::window().expect("no global `window` exists");
                                            let location = window.location();
                                            let _ = location.set_href(&auth_url);
                                        }
                                    },
                                    "Connect"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
