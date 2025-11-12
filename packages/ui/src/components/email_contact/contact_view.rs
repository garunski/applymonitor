//! Contact view mode component

use crate::components::button::{Button, ButtonVariant};
use crate::services::jobs_service::EmailContact;
use dioxus::prelude::*;

/// Contact view mode component
#[component]
pub fn ContactView(contact: EmailContact, on_edit: EventHandler) -> Element {
    rsx! {
        div {
            class: "space-y-4",
            if let Some(ref n) = contact.name {
                if !n.is_empty() {
                    div {
                        class: "space-y-1",
                        p {
                            class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                            "Name"
                        }
                        p {
                            class: "text-sm text-gray-900 dark:text-white",
                            {n.clone()}
                        }
                    }
                }
            }
            if let Some(ref l) = contact.linkedin {
                if !l.is_empty() {
                    div {
                        class: "space-y-1",
                        p {
                            class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                            "LinkedIn"
                        }
                        a {
                            href: l.clone(),
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-sm text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300",
                            {l.clone()}
                        }
                    }
                }
            }
            if let Some(ref w) = contact.website {
                if !w.is_empty() {
                    div {
                        class: "space-y-1",
                        p {
                            class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase",
                            "Website"
                        }
                        a {
                            href: w.clone(),
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-sm text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300",
                            {w.clone()}
                        }
                    }
                }
            }
            if contact.name.is_none() && contact.linkedin.is_none() && contact.website.is_none() {
                p {
                    class: "text-sm text-gray-400 dark:text-gray-500 italic",
                    "No additional information available"
                }
            }
            div {
                class: "pt-4",
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| on_edit.call(()),
                    "Edit"
                }
            }
        }
    }
}
