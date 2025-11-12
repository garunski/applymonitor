//! Job selection dialog component for assigning emails to jobs

use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use crate::state::{use_emails, use_jobs};
use dioxus::prelude::*;

/// Job selection dialog component
#[component]
pub fn JobSelectDialog(open: Signal<bool>, gmail_id: String) -> Element {
    let jobs_state = use_jobs();
    let emails_state = use_emails();

    // Fetch jobs when dialog opens
    use_effect({
        let open_signal = open;
        let jobs = jobs_state;
        move || {
            if *open_signal.read() {
                jobs.fetch_jobs();
            }
        }
    });

    let mut open_option = use_signal(|| Some(*open.read()));
    use_effect(move || {
        *open_option.write() = Some(*open.read());
    });

    // Filter to only show jobs with status_id 100 (open)
    let open_jobs: Vec<_> = jobs_state
        .jobs
        .read()
        .iter()
        .filter(|j| j.status_id == Some(100))
        .cloned()
        .collect();

    rsx! {
        DialogRoot {
            open: open_option,
            DialogContent {
                DialogTitle {
                    "Assign to Job"
                }
                DialogDescription {
                    "Select a job to assign this email to."
                }
                div {
                    class: "mt-4 max-h-[400px] overflow-y-auto",
                    if *jobs_state.loading.read() {
                        div {
                            class: "text-center py-8",
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "Loading jobs..."
                            }
                        }
                    } else if open_jobs.is_empty() {
                        div {
                            class: "text-center py-8",
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "No open jobs available."
                            }
                        }
                    } else {
                        ul {
                            class: "space-y-2",
                            for job in open_jobs {
                                li {
                                    button {
                                        class: "w-full text-left px-4 py-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors",
                                        onclick: {
                                            let gmail_id_clone = gmail_id.clone();
                                            let job_id = job.id.clone();
                                            let mut open_signal = open;
                                            let emails = emails_state;
                                            move |_| {
                                                if let Some(id) = job_id.clone() {
                                                    emails.assign_to_existing_job(gmail_id_clone.clone(), id);
                                                    *open_signal.write() = false;
                                                }
                                            }
                                        },
                                        div {
                                            class: "flex items-center gap-3",
                                            // Company initial avatar
                                            div {
                                                class: "size-10 flex-none rounded-full bg-brand-100 dark:bg-brand-900 flex items-center justify-center dark:outline dark:outline-1 dark:-outline-offset-1 dark:outline-white/10",
                                                span {
                                                    class: "text-brand-600 dark:text-brand-400 font-medium text-sm",
                                                    {job.company.chars().next().unwrap_or('?').to_uppercase().collect::<String>()}
                                                }
                                            }
                                            // Job details
                                            div {
                                                class: "min-w-0 flex-auto",
                                                p {
                                                    class: "text-sm font-semibold text-gray-900 dark:text-white",
                                                    {job.title.clone()}
                                                }
                                                p {
                                                    class: "text-xs text-gray-500 dark:text-gray-400 mt-0.5",
                                                    {job.company.clone()}
                                                    if let Some(ref loc) = job.location {
                                                        " • {loc}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "flex justify-end mt-6 pt-4 border-t border-gray-200 dark:border-gray-700",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| *open.write() = false,
                        "Cancel"
                    }
                }
            }
        }
    }
}
