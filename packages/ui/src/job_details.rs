//! Job details component

use crate::comment_form::CommentForm;
use crate::components::status_change_dialog::StatusChangeDialog;
use crate::email_contact_card::EmailContactCard;
use crate::email_contact_slideout::EmailContactSlideout;
use crate::job_details_components::{DetailsTab, EmailsTab, JobDetailsHeader};
use crate::state::{use_comments_provider, use_email_contacts_provider, use_emails, use_jobs};
use crate::timeline::Timeline;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
enum DetailsTabType {
    Details,
    Emails,
}

/// Job details component
#[component]
pub fn JobDetails(job_id: String) -> Element {
    let jobs_state = use_jobs();
    let email_contacts_state = use_email_contacts_provider();
    let comments_state = use_comments_provider();
    let emails_state = use_emails();

    // Editing state for each field
    let editing_title = use_signal(|| false);
    let editing_company = use_signal(|| false);
    let editing_location = use_signal(|| false);
    let editing_status = use_signal(|| false);
    let editing_description = use_signal(|| false);

    // Edit values
    let edit_title_value = use_signal(String::new);
    let edit_company_value = use_signal(String::new);
    let edit_location_value = use_signal(String::new);
    let edit_status_value = use_signal(String::new);
    let edit_description_value = use_signal(String::new);

    // Tab state
    let active_tab = use_signal(|| DetailsTabType::Details);

    // Dialog state
    let show_dialog = use_signal(|| false);
    let selected_status_id = use_signal(|| None::<i32>);

    // Fetch job statuses on mount
    use_effect({
        let jobs_state_statuses = jobs_state;
        move || {
            jobs_state_statuses.fetch_job_statuses();
        }
    });

    // Fetch job details on mount
    use_effect({
        let job_id_clone = job_id.clone();
        let jobs_state_fetch = jobs_state;
        let contacts_state = email_contacts_state;
        let emails = emails_state;
        let comments = comments_state;
        move || {
            let job_id_for_fetch = job_id_clone.clone();
            jobs_state_fetch.fetch_job_details(job_id_for_fetch, contacts_state, emails, comments);
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
        let job = details.job.clone();
        let timeline_events = details.timeline_events.clone();
        let contacts = email_contacts_state.contacts.read().clone();
        let emails = emails_state.emails.read().clone();
        let statuses = jobs_state.job_statuses.read().clone();
        let current_status_id = job.status_id;

        // Find current and new status for dialog
        let current_status = statuses
            .iter()
            .find(|s| s.id == current_status_id.unwrap_or(0))
            .cloned();
        let new_status =
            selected_status_id().and_then(|id| statuses.iter().find(|s| s.id == id).cloned());

        rsx! {
            div {
                class: "px-4 sm:px-6 lg:px-8 py-6",
                JobDetailsHeader {
                    job_id: job_id.clone(),
                    title: job.title.clone(),
                    company: job.company.clone(),
                    location: job.location.clone(),
                    status_id: current_status_id,
                    statuses: statuses.clone(),
                    on_status_click: move |id| {
                        let mut selected = selected_status_id;
                        *selected.write() = Some(id);
                        let mut show = show_dialog;
                        *show.write() = true;
                    },
                    editing_title,
                    editing_company,
                    editing_location,
                    edit_title_value,
                    edit_company_value,
                    edit_location_value,
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
                            events: timeline_events,
                        }
                        div {
                            class: "mt-6",
                            CommentForm {
                                job_id: job_id.clone(),
                            }
                        }
                    }

                    // Right column: Job details + Emails
                    div {
                        class: "space-y-6",
                        // Details/Emails card with tabs
                        div {
                            class: "rounded-lg bg-white dark:bg-gray-800 p-5 ring-1 ring-inset ring-gray-200 dark:ring-white/15",
                            // Tabs
                            div {
                                class: "mb-4 border-b border-gray-200 dark:border-gray-700",
                                nav {
                                    class: "-mb-px flex space-x-8",
                                    button {
                                        class: if *active_tab.read() == DetailsTabType::Details {
                                            "border-indigo-500 text-indigo-600 dark:text-indigo-400 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium"
                                        } else {
                                            "border-transparent text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:hover:border-gray-600 hover:text-gray-700 dark:hover:text-gray-300 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium"
                                        },
                                        onclick: move |_| {
                                            let mut tab = active_tab;
                                            *tab.write() = DetailsTabType::Details;
                                        },
                                        "Details"
                                    }
                                    button {
                                        class: if *active_tab.read() == DetailsTabType::Emails {
                                            "border-indigo-500 text-indigo-600 dark:text-indigo-400 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium"
                                        } else {
                                            "border-transparent text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:hover:border-gray-600 hover:text-gray-700 dark:hover:text-gray-300 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium"
                                        },
                                        onclick: move |_| {
                                            let mut tab = active_tab;
                                            *tab.write() = DetailsTabType::Emails;
                                        },
                                        "Emails"
                                    }
                                }
                            }

                            // Tab content
                            if *active_tab.read() == DetailsTabType::Details {
                                DetailsTab {
                                    job_id: job_id.clone(),
                                    status_id: job.status_id,
                                    created_at: job.created_at.clone(),
                                    updated_at: job.updated_at.clone(),
                                    description: job.description.clone(),
                                    editing_status,
                                    editing_description,
                                    edit_status_value,
                                    edit_description_value,
                                }
                            } else {
                                EmailsTab {
                                    job_id: job_id.clone(),
                                    emails,
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

            // Status change dialog
            if *show_dialog.read() {
                if let (Some(current), Some(new)) = (current_status, new_status) {
                    StatusChangeDialog {
                        current_status: current.clone(),
                        new_status: new.clone(),
                        on_confirm: move |_| {
                            if let Some(new_id) = selected_status_id() {
                                jobs_state.update_job_status(job_id.clone(), new_id);
                            }
                            let mut show = show_dialog;
                            *show.write() = false;
                            let mut selected = selected_status_id;
                            *selected.write() = None;
                        },
                        on_cancel: move |_| {
                            let mut show = show_dialog;
                            *show.write() = false;
                            let mut selected = selected_status_id;
                            *selected.write() = None;
                        },
                    }
                }
            }
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
