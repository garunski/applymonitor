//! Slideout header component for email contact

use crate::components::button::{Button, ButtonVariant};
use crate::services::jobs_service::EmailContact;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsX;
use dioxus_free_icons::Icon;

/// Slideout header component
#[component]
pub fn SlideoutHeader(
    contact: EmailContact,
    display_name: String,
    on_close: EventHandler,
) -> Element {
    let is_system = contact.is_system;

    rsx! {
        div {
            class: "sticky top-0 z-10 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 px-6 py-4",
            div {
                class: "flex items-start justify-between mb-4",
                div {
                    class: "flex-1 min-w-0",
                    h2 {
                        class: "text-lg font-semibold text-gray-900 dark:text-white pr-4 truncate",
                        {display_name}
                    }
                    p {
                        class: "text-sm text-gray-500 dark:text-gray-400 mt-1 truncate",
                        {contact.email.clone()}
                    }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    class: "shrink-0",
                    onclick: move |_| on_close.call(()),
                    Icon {
                        class: "w-5 h-5",
                        width: 20,
                        height: 20,
                        fill: "currentColor",
                        icon: BsX,
                    }
                }
            }

            // Saved contact badge
            if is_system {
                div {
                    class: "mb-4",
                    span {
                        class: "inline-flex items-center rounded-md bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 text-sm font-medium text-indigo-700 dark:text-indigo-300 ring-1 ring-inset ring-indigo-700/10 dark:ring-indigo-300/20",
                        "Saved Contact"
                    }
                }
            }
        }
    }
}
