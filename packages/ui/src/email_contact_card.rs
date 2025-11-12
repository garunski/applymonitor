//! Email contact card component

use crate::services::jobs_service::EmailContact;
use crate::state::use_email_contacts;
use dioxus::prelude::*;

/// Email contact card component
#[component]
pub fn EmailContactCard(contact: EmailContact) -> Element {
    let contacts_state = use_email_contacts();

    let display_name = contact
        .name
        .clone()
        .unwrap_or_else(|| contact.email.clone());

    let initials = get_initials(&display_name);
    let is_system = contact.is_system;

    rsx! {
        li {
            class: "flex items-center gap-x-3 p-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800/50 cursor-pointer transition-colors",
            onclick: {
                let contact_clone = contact.clone();
                move |_| {
                    contacts_state.select_contact(contact_clone.clone());
                }
            },
            // Avatar
            div {
                class: if is_system {
                    "size-10 flex-none rounded-full bg-indigo-100 dark:bg-indigo-900/30 flex items-center justify-center"
                } else {
                    "size-10 flex-none rounded-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center"
                },
                if is_system {
                    span {
                        class: "text-xs font-medium text-indigo-600 dark:text-indigo-400",
                        "⚙"
                    }
                } else {
                    span {
                        class: "text-xs font-medium text-gray-600 dark:text-gray-300",
                        {initials}
                    }
                }
            }
            // Contact info
            div {
                class: "min-w-0 flex-auto",
                div {
                    class: "flex items-center gap-x-2",
                    p {
                        class: "text-sm font-medium text-gray-900 dark:text-white",
                        {display_name}
                    }
                    if is_system {
                        span {
                            class: "inline-flex items-center rounded-md bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 text-xs font-medium text-indigo-700 dark:text-indigo-300 ring-1 ring-inset ring-indigo-700/10 dark:ring-indigo-300/20",
                            "Saved"
                        }
                    }
                }
                p {
                    class: "text-xs text-gray-500 dark:text-gray-400 truncate",
                    {contact.email.clone()}
                }
            }
        }
    }
}

/// Get initials from a name
fn get_initials(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    match parts.len() {
        0 => "?".to_string(),
        1 => parts[0]
            .chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_else(|| "?".to_string()),
        _ => {
            let first = parts[0]
                .chars()
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default();
            let last = parts[parts.len() - 1]
                .chars()
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default();
            format!("{}{}", first, last)
        }
    }
}
