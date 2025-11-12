//! Job details component

use crate::comment_form::CommentForm;
use crate::email_contact_card::EmailContactCard;
use crate::email_contact_slideout::EmailContactSlideout;
use crate::state::{use_email_contacts_provider, use_jobs};
use crate::timeline::Timeline;
use dioxus::prelude::*;

/// Job details component
#[component]
pub fn JobDetails(job_id: String) -> Element {
    let jobs_state = use_jobs();
    let email_contacts_state = use_email_contacts_provider();
    let mut description = use_signal(String::new);
    let mut is_editing_description = use_signal(|| false);
    let saving_description = use_signal(|| false);

    // Fetch job details on mount
    use_effect({
        let job_id_clone = job_id.clone();
        let jobs_state_fetch = jobs_state;
        move || {
            let job_id_for_fetch = job_id_clone.clone();
            jobs_state_fetch.fetch_job_details(job_id_for_fetch);
        }
    });

    // Update description and contacts when job details load
    use_effect({
        let jobs_state_desc = jobs_state;
        let email_contacts_state_effect = email_contacts_state;
        let mut desc_signal = description;
        move || {
            if let Some(details) = jobs_state_desc.selected_job.read().as_ref() {
                *desc_signal.write() = details.job.description.clone().unwrap_or_default();
                email_contacts_state_effect.set_contacts(details.contacts.clone());
            }
        }
    });

    let job_details = jobs_state.selected_job.read().clone();

    if *jobs_state.loading.read() && job_details.is_none() {
        return rsx! {
            div {
                class: "px-4 sm:px-6 lg:px-8 py-6",
                div {
                    class: "text-center",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "Loading job details..."
                    }
                }
            }
        };
    }

    if let Some(details) = job_details {
        let job = &details.job;
        let timeline_events = &details.timeline_events;
        let contacts = &details.contacts;

        rsx! {
            div {
                class: "px-4 sm:px-6 lg:px-8 py-6",
                // Header
                div {
                    class: "mb-6",
                    h1 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white",
                        {job.title.clone()}
                    }
                    p {
                        class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                        {job.company.clone()}
                        if let Some(ref loc) = job.location {
                            " • {loc}"
                        }
                    }
                }

                // Two-column layout (stacked on mobile)
                div {
                    class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                    // Left column: Timeline
                    div {
                        class: "lg:col-span-2",
                        h2 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                            "Timeline"
                        }
                        Timeline {
                            events: timeline_events.clone(),
                        }
                        div {
                            class: "mt-6",
                            CommentForm {
                                job_id: job_id.clone(),
                            }
                        }
                    }

                    // Right column: Job details + People
                    div {
                        class: "space-y-6",
                        // Job details card
                        div {
                            class: "rounded-lg bg-white dark:bg-gray-800 p-5 ring-1 ring-inset ring-gray-200 dark:ring-white/15",
                            h2 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                                "Details"
                            }
                            div {
                                class: "space-y-4",
                                div {
                                    class: "flex justify-between",
                                    span {
                                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                        "Status"
                                    }
                                    span {
                                        class: "text-sm text-gray-900 dark:text-white",
                                        {job.status.chars().next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default()}
                                        {job.status.chars().skip(1).collect::<String>()}
                                    }
                                }
                                if let Some(ref created_at) = job.created_at {
                                    div {
                                        class: "flex justify-between",
                                        span {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                            "Created"
                                        }
                                        span {
                                            class: "text-sm text-gray-900 dark:text-white",
                                            {format_date(created_at)}
                                        }
                                    }
                                }
                                if let Some(ref updated_at) = job.updated_at {
                                    div {
                                        class: "flex justify-between",
                                        span {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                            "Updated"
                                        }
                                        span {
                                            class: "text-sm text-gray-900 dark:text-white",
                                            {format_date(updated_at)}
                                        }
                                    }
                                }
                            }

                            // Description section
                            div {
                                class: "mt-4 pt-4 border-t border-gray-200 dark:border-gray-700",
                                div {
                                    class: "flex justify-between items-center mb-2",
                                    label {
                                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                        "Description"
                                    }
                                    if !is_editing_description() {
                                        button {
                                            class: "text-xs text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300",
                                            onclick: move |_| *is_editing_description.write() = true,
                                            "Edit"
                                        }
                                    }
                                }
                                if is_editing_description() {
                                    div {
                                        class: "space-y-2",
                                        textarea {
                                            class: "block w-full rounded-md border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            rows: "4",
                                            value: "{description}",
                                            oninput: move |e: Event<FormData>| *description.write() = e.value(),
                                        }
                                        div {
                                            class: "flex justify-end gap-2",
                                            button {
                                                class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                                                onclick: {
                                                    let mut is_editing = is_editing_description;
                                                    let mut desc = description;
                                                    let jobs_state_clone = jobs_state;
                                                    move |_| {
                                                        *is_editing.write() = false;
                                                        // Reset to original value
                                                        if let Some(details) = jobs_state_clone.selected_job.read().as_ref() {
                                                            *desc.write() = details.job.description.clone().unwrap_or_default();
                                                        }
                                                    }
                                                },
                                                "Cancel"
                                            }
                                            button {
                                                class: "text-sm text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300",
                                                onclick: {
                                                    let job_id_clone = job_id.clone();
                                                    let jobs_state_clone = jobs_state;
                                                    let is_editing = is_editing_description;
                                                    let saving = saving_description;
                                                    let desc_signal = description;

                                                    move |_| {
                                                        let desc_value = desc_signal().trim().to_string();
                                                        let desc_opt = if desc_value.is_empty() {
                                                            None
                                                        } else {
                                                            Some(desc_value)
                                                        };

                                                        let job_id_for_update = job_id_clone.clone();
                                                        let mut saving_clone = saving;
                                                        let mut is_editing_clone = is_editing;

                                                        spawn(async move {
                                                            *saving_clone.write() = true;
                                                            jobs_state_clone.update_job_description(job_id_for_update, desc_opt);
                                                            *saving_clone.write() = false;
                                                            *is_editing_clone.write() = false;
                                                        });
                                                    }
                                                },
                                                disabled: saving_description(),
                                                if saving_description() {
                                                    "Saving..."
                                                } else {
                                                    "Save"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if description().is_empty() {
                                        p {
                                            class: "text-sm text-gray-400 dark:text-gray-500 italic",
                                            "No description"
                                        }
                                    } else {
                                        p {
                                            class: "text-sm text-gray-900 dark:text-white whitespace-pre-wrap",
                                            {description()}
                                        }
                                    }
                                }
                            }
                        }

                        // Email contacts section
                        div {
                            class: "rounded-lg bg-white dark:bg-gray-800 p-5 ring-1 ring-inset ring-gray-200 dark:ring-white/15",
                            h2 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                                "Email Contacts"
                            }
                            if contacts.is_empty() {
                                p {
                                    class: "text-sm text-gray-400 dark:text-gray-500 italic",
                                    "No email contacts found. Link emails to this job to see contacts."
                                }
                            } else {
                                ul {
                                    role: "list",
                                    class: "space-y-2",
                                    for contact in contacts.iter() {
                                        EmailContactCard {
                                            contact: contact.clone(),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Email contact slideout
            EmailContactSlideout {}
        }
    } else {
        rsx! {
            div {
                class: "px-4 sm:px-6 lg:px-8 py-6",
                div {
                    class: "text-center",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "Job not found"
                    }
                }
            }
        }
    }
}

fn format_date(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt.format("%B %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}
