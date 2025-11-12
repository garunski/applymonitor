//! Emails tab component

use crate::services::emails_service::{EmailsService, StoredEmail};
use crate::state::{use_comments_provider, use_email_contacts_provider, use_emails, use_jobs};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsX;
use dioxus_free_icons::Icon;

#[component]
pub fn EmailsTab(job_id: String, emails: Vec<StoredEmail>) -> Element {
    let jobs_state = use_jobs();
    let email_contacts_state = use_email_contacts_provider();
    let emails_state = use_emails();
    let comments_state = use_comments_provider();

    rsx! {
        div {
            if emails.is_empty() {
                p {
                    class: "text-sm text-gray-400 dark:text-gray-500 italic",
                    "No emails linked to this job."
                }
            } else {
                ul {
                    role: "list",
                    class: "divide-y divide-gray-100 dark:divide-white/5",
                    for email in emails.iter() {
                        li {
                            class: "flex justify-between gap-x-6 py-4",
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
                                }
                            }
                            button {
                                class: "flex-shrink-0 text-gray-400 hover:text-red-600 dark:hover:text-red-400",
                                onclick: {
                                    let email_id = email.gmail_id.clone();
                                    let job_id_clone = job_id.clone();
                                    let jobs_state_unassign = jobs_state;
                                    let contacts_state_unassign = email_contacts_state;
                                    let emails_state_unassign = emails_state;
                                    let comments_state_unassign = comments_state;
                                    move |_| {
                                        let email_id_clone = email_id.clone();
                                        let job_id_unassign = job_id_clone.clone();
                                        spawn(async move {
                                            if EmailsService::unassign_email_from_job(email_id_clone).await.is_ok() {
                                                jobs_state_unassign.fetch_job_details(
                                                    job_id_unassign,
                                                    contacts_state_unassign,
                                                    emails_state_unassign,
                                                    comments_state_unassign,
                                                );
                                            }
                                        });
                                    }
                                },
                                Icon {
                                    class: "h-5 w-5",
                                    width: 20,
                                    height: 20,
                                    fill: "currentColor",
                                    icon: BsX,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
