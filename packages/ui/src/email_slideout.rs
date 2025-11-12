//! Email slideout component for displaying email details

use crate::components::button::{Button, ButtonVariant};
use crate::job_form::JobForm;
use crate::job_select_dialog::JobSelectDialog;
use crate::state::{use_auth, use_emails};
use crate::utils::format_date_full;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsX;
use dioxus_free_icons::Icon;

/// Email slideout component
#[component]
pub fn EmailSlideout() -> Element {
    let auth = use_auth();
    let emails_state = use_emails();
    let mut show_job_form = use_signal(|| false);
    let mut show_job_select = use_signal(|| false);

    let selected_email = emails_state.selected_email.read().clone();

    if let Some(email) = selected_email {
        // Extract company name from "from" field (e.g., "Company Name <email@example.com>")
        let company_name = email
            .from
            .as_ref()
            .map(|f| {
                let name = if let Some(start) = f.find('<') {
                    f[..start].trim().to_string()
                } else {
                    f.clone()
                };
                // Remove quotes if present
                name.trim_matches(|c| c == '"' || c == '\'')
                    .trim()
                    .to_string()
            })
            .unwrap_or_else(|| "Unknown Company".to_string());

        // Use subject as title, or default
        let job_title = email
            .subject
            .as_ref()
            .unwrap_or(&"New Job Application".to_string())
            .clone();

        // Compute timezone and formatted date before RSX
        let user = auth.user.read();
        let timezone = user.as_ref().and_then(|u| u.timezone.as_deref());
        let formatted_date = email.date.as_ref().map(|d| format_date_full(d, timezone));

        rsx! {
            // Backdrop
            div {
                class: "fixed inset-0 z-50 bg-black/50 transition-opacity",
                onclick: move |_| {
                    emails_state.clear_selected();
                },
            }

            // Slideout panel
            div {
            class: "fixed right-0 top-0 bottom-0 z-50 w-full sm:w-[600px] bg-white dark:bg-gray-900 shadow-xl transform transition-transform duration-300 ease-in-out overflow-y-auto",
            div {
                class: "flex flex-col h-full",
                // Header with badges and close button
                div {
                    class: "sticky top-0 z-10 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 px-6 py-4",
                    div {
                        class: "flex items-start justify-between mb-4",
                        h2 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white pr-4",
                            {email.subject.clone().unwrap_or_else(|| "No subject".to_string())}
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            class: "shrink-0",
                            onclick: move |_| {
                                emails_state.clear_selected();
                            },
                            Icon {
                                class: "w-5 h-5",
                                width: 20,
                                height: 20,
                                fill: "currentColor",
                                icon: BsX,
                            }
                        }
                    }

                    // Badges section (empty for now, ready for AI)
                    div {
                        class: "flex flex-wrap gap-2 mb-4",
                        // Placeholder for AI-generated badges
                        // Will be populated later with badges like "Application", "Interview", etc.
                    }

                    // Assign and Create Job buttons
                    div {
                        class: "flex gap-3",
                        Button {
                            variant: ButtonVariant::Secondary,
                            class: "flex-1 sm:flex-none",
                            onclick: move |_| {
                                *show_job_select.write() = true;
                            },
                            "Assign"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            class: "flex-1 sm:flex-none",
                            onclick: move |_| {
                                *show_job_form.write() = true;
                            },
                            "Create"
                        }
                    }
                }

                // Email content
                div {
                    class: "flex-1 px-6 py-4 space-y-4",
                    // From
                    div {
                        class: "space-y-1",
                        p {
                            class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                            "From"
                        }
                        p {
                            class: "text-sm text-gray-900 dark:text-white",
                            {email.from.clone().unwrap_or_else(|| "Unknown".to_string())}
                        }
                    }

                    // To
                    if let Some(ref to) = email.to {
                        div {
                            class: "space-y-1",
                            p {
                                class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                "To"
                            }
                            p {
                                class: "text-sm text-gray-900 dark:text-white",
                                {to.clone()}
                            }
                        }
                    }

                    // Date
                    if let Some(ref formatted_date) = formatted_date {
                        div {
                            class: "space-y-1",
                            p {
                                class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                "Date"
                            }
                            p {
                                class: "text-sm text-gray-900 dark:text-white",
                                {formatted_date.clone()}
                            }
                        }
                    }

                    // Snippet/Body
                    if let Some(ref snippet) = email.snippet {
                        if !snippet.is_empty() {
                            div {
                                class: "space-y-1 pt-4 border-t border-gray-200 dark:border-gray-700",
                                p {
                                    class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                    "Preview"
                                }
                                p {
                                    class: "text-sm text-gray-900 dark:text-white whitespace-pre-wrap",
                                    {snippet.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }

        // Job selection dialog
        JobSelectDialog {
            open: show_job_select,
            gmail_id: email.gmail_id.clone(),
        }

        // Job form dialog
        JobForm {
            open: show_job_form,
            job: None,
            prefill_title: Some(job_title),
            prefill_company: Some(company_name),
            gmail_id: Some(email.gmail_id.clone()),
        }
        }
    } else {
        rsx! { div {} }
    }
}
