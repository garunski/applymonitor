//! Emails list component

use crate::components::button::{Button, ButtonVariant};
use crate::email_slideout::EmailSlideout;
use crate::services::gmail_scanner_service::GmailScannerService;
use crate::state::{use_auth, use_emails, use_emails_provider};
use crate::utils::format_date;
use dioxus::prelude::*;

/// Emails list component
#[component]
pub fn EmailsList() -> Element {
    use_emails_provider();
    let auth = use_auth();
    let emails_state = use_emails();
    let scanning = use_signal(|| false);
    let scan_error = use_signal(|| None::<String>);

    // Compute timezone once for all emails
    let user = auth.user.read();
    let timezone = user.as_ref().and_then(|u| u.timezone.as_deref());

    // Fetch emails on mount
    use_effect(move || {
        emails_state.fetch_emails(Some(100), Some(0));
    });

    let handle_scan = {
        let scanning_signal = scanning;
        let error_signal = scan_error;
        let emails = emails_state;

        move |_| {
            let mut scanning_clone = scanning_signal;
            let mut error_clone = error_signal;
            let emails_clone = emails;

            spawn(async move {
                *scanning_clone.write() = true;
                *error_clone.write() = None;

                match GmailScannerService::scan_emails(None, None).await {
                    Ok(_) => {
                        // Refresh emails list after successful scan
                        emails_clone.fetch_emails(Some(100), Some(0));
                        *error_clone.write() = None;
                    }
                    Err(e) => {
                        *error_clone.write() = Some(format!("Failed to scan Gmail: {}", e));
                    }
                }

                *scanning_clone.write() = false;
            });
        }
    };

    rsx! {
        div {
            class: "px-4 sm:px-6 lg:px-8 py-6",
            div {
                class: "sm:flex sm:items-center sm:justify-between",
                div {
                    class: "sm:flex-auto",
                    h1 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white",
                        "Emails"
                    }
                    p {
                        class: "mt-2 text-sm text-gray-700 dark:text-gray-300",
                        "Scanned emails from your Gmail account."
                    }
                }
                div {
                    class: "mt-4 sm:ml-16 sm:mt-0 sm:flex-none",
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: handle_scan,
                        disabled: scanning(),
                        if scanning() {
                            "Scanning..."
                        } else {
                            "Scan Gmail"
                        }
                    }
                }
            }

            // Scan error message
            if let Some(err) = scan_error.read().as_ref() {
                div {
                    class: "mt-4 rounded-md bg-red-50 dark:bg-red-900/20 p-4",
                    p {
                        class: "text-sm text-red-800 dark:text-red-200",
                        {err.clone()}
                    }
                }
            }

            // Loading state
            if *emails_state.loading.read() {
                div {
                    class: "mt-6 text-center",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "Loading emails..."
                    }
                }
            }

            // Error state
            if let Some(error) = emails_state.error.read().as_ref() {
                div {
                    class: "mt-6 rounded-md bg-red-50 dark:bg-red-900/20 p-4",
                    div {
                        class: "flex",
                        div {
                            class: "ml-3",
                            h3 {
                                class: "text-sm font-medium text-red-800 dark:text-red-200",
                                "Error loading emails"
                            }
                            div {
                                class: "mt-2 text-sm text-red-700 dark:text-red-300",
                                "{error}"
                            }
                        }
                    }
                }
            }

            // Emails list
            if !*emails_state.loading.read() && emails_state.error.read().is_none() {
                div {
                    class: "mt-6 flow-root",
                    ul {
                        role: "list",
                        class: "divide-y divide-gray-100 dark:divide-white/5",
                        for email in emails_state.emails.read().clone() {
                            li {
                                class: "flex justify-between gap-x-6 py-4 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800/50 rounded-md px-2 -mx-2",
                                onclick: move |_| {
                                    emails_state.select_email(email.clone());
                                },
                                div {
                                    class: "flex min-w-0 gap-x-4 flex-1",
                                    div {
                                        class: "min-w-0 flex-auto",
                                        p {
                                            class: "text-sm font-semibold leading-6 text-gray-900 dark:text-white",
                                            {email.subject.clone().unwrap_or_else(|| "No subject".to_string())}
                                        }
                                        p {
                                            class: "mt-1 truncate text-xs leading-5 text-gray-500 dark:text-gray-400",
                                            {email.from.clone().unwrap_or_else(|| "Unknown sender".to_string())}
                                        }
                                        if let Some(snippet) = &email.snippet {
                                            if !snippet.is_empty() {
                                                p {
                                                    class: "mt-1 text-xs text-gray-600 dark:text-gray-400 line-clamp-2",
                                                    {snippet.clone()}
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "hidden sm:flex sm:flex-col sm:items-end",
                                    if let Some(ref date) = email.date {
                                        p {
                                            class: "text-xs leading-6 text-gray-900 dark:text-white",
                                            {format_date(date, timezone)}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if emails_state.emails.read().is_empty() {
                        div {
                            class: "text-center py-8",
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "No emails found. Scan your Gmail to get started!"
                            }
                        }
                    }
                }
            }
        }

        // Email slideout
        EmailSlideout {}
    }
}
