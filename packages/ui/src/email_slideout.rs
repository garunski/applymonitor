//! Email slideout component for displaying email details

#[allow(unused_imports)]
use crate::components::button::{Button, ButtonVariant};
#[allow(unused_imports)]
use crate::job_form::JobForm;
#[allow(unused_imports)]
use crate::job_select_dialog::JobSelectDialog;
use crate::services::ai_service::AiService;
use crate::state::{use_auth, use_emails};
use crate::utils::format_date_full;
use dioxus::prelude::*;
#[allow(unused_imports)]
use dioxus_free_icons::icons::bs_icons::BsX;
#[allow(unused_imports)]
use dioxus_free_icons::Icon;

/// Email slideout component
#[component]
pub fn EmailSlideout() -> Element {
    let auth = use_auth();
    let emails_state = use_emails();
    let mut show_job_form = use_signal(|| false);
    let mut show_job_select = use_signal(|| false);
    let ai_result = use_signal(|| None::<crate::services::ai_service::AiResult>);
    let loading_ai = use_signal(|| false);
    let processing = use_signal(|| false);

    let selected_email = emails_state.selected_email.read().clone();
    let selected_email_clone = selected_email.clone();

    // Fetch AI results when email is selected
    use_effect(move || {
        let email_id = selected_email_clone.as_ref().map(|e| e.gmail_id.clone());
        let mut ai_result_signal = ai_result;
        let mut loading_signal = loading_ai;

        if let Some(email_id) = email_id {
            spawn(async move {
                *loading_signal.write() = true;
                match AiService::get_ai_results(&email_id).await {
                    Ok(result) => {
                        *ai_result_signal.write() = Some(result);
                    }
                    Err(_) => {
                        *ai_result_signal.write() = None;
                    }
                }
                *loading_signal.write() = false;
            });
        } else {
            *ai_result_signal.write() = None;
        }
    });

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

        // Compute AI confidence text before RSX
        let ai_conf_text = ai_result().as_ref().and_then(|ai| {
            ai.confidence
                .map(|c| format!("Confidence: {:.0}%", c * 100.0))
        });

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

                    // AI badges section
                    if let Some(ref ai) = ai_result() {
                        div {
                            class: "flex flex-wrap gap-2 mb-4",
                            if let Some(ref category) = ai.category {
                                span {
                                    class: "inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200",
                                    {category.clone()}
                                }
                            }
                            if let Some(ref conf_text) = ai_conf_text {
                                span {
                                    class: "inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200",
                                    {conf_text.clone()}
                                }
                            }
                            if let Some(conf) = ai.confidence {
                                if conf < 0.7 {
                                    span {
                                        class: "inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-yellow-100 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-200",
                                        "Needs Review"
                                    }
                                }
                            }
                        }
                    } else if loading_ai() {
                        div {
                            class: "flex flex-wrap gap-2 mb-4",
                            span {
                                class: "text-sm text-gray-500 dark:text-gray-400",
                                "Loading AI analysis..."
                            }
                        }
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

                    // AI Results Section
                    div {
                        class: "space-y-4 pt-4 border-t border-gray-200 dark:border-gray-700",
                        h3 {
                            class: "text-sm font-medium text-gray-900 dark:text-white",
                            "AI Analysis"
                        }
                        if let Some(ref ai) = ai_result() {
                            if let Some(ref summary) = ai.summary {
                                div {
                                    class: "space-y-1",
                                    p {
                                        class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                        "Summary"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 dark:text-white",
                                        {summary.clone()}
                                    }
                                }
                            }
                            if let Some(ref company) = ai.company {
                                div {
                                    class: "space-y-1",
                                    p {
                                        class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                        "Company"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 dark:text-white",
                                        {company.clone()}
                                    }
                                }
                            }
                            if let Some(ref job_title) = ai.job_title {
                                div {
                                    class: "space-y-1",
                                    p {
                                        class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                        "Job Title"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 dark:text-white",
                                        {job_title.clone()}
                                    }
                                }
                            }
                            if let Some(ref extracted_data) = ai.extracted_data {
                                if let Ok(data) = serde_json::from_str::<serde_json::Value>(extracted_data) {
                                    div {
                                        class: "space-y-1",
                                        p {
                                            class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                                            "Extracted Data"
                                        }
                                        pre {
                                            class: "text-xs text-gray-600 dark:text-gray-400 bg-gray-50 dark:bg-gray-800 p-2 rounded",
                                            {serde_json::to_string_pretty(&data).unwrap_or_default()}
                                        }
                                    }
                                }
                            }
                        } else if *loading_ai.read() {
                            p {
                                class: "text-sm text-gray-500 dark:text-gray-400",
                                "Loading AI results..."
                            }
                        } else {
                            p {
                                class: "text-sm text-gray-500 dark:text-gray-400",
                                "No AI analysis available. Click the button below to process this email."
                            }
                        }
                        div {
                            class: "pt-2",
                            Button {
                                variant: ButtonVariant::Secondary,
                                disabled: processing(),
                                onclick: {
                                    let email_id = email.gmail_id.clone();
                                    let user_id = auth.user.read().as_ref().map(|u| u.id.clone()).unwrap_or_default();
                                    let mut processing_signal = processing;
                                    let mut ai_result_signal = ai_result;
                                    let mut loading_signal = loading_ai;
                                    move |_| {
                                        let email_id = email_id.clone();
                                        let user_id = user_id.clone();
                                        spawn(async move {
                                            *processing_signal.write() = true;
                                            if AiService::process_email(&email_id, &user_id).await.is_ok() {
                                                // Reload AI results
                                                *loading_signal.write() = true;
                                                match AiService::get_ai_results(&email_id).await {
                                                    Ok(result) => {
                                                        *ai_result_signal.write() = Some(result);
                                                    }
                                                    Err(_) => {
                                                        *ai_result_signal.write() = None;
                                                    }
                                                }
                                                *loading_signal.write() = false;
                                            }
                                            *processing_signal.write() = false;
                                        });
                                    }
                                },
                                if processing() {
                                    "Processing..."
                                } else {
                                    "Process with AI"
                                }
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
