//! System email detection banner component

use crate::components::button::Button;
use crate::components::button::ButtonVariant;
use crate::services::jobs_service::EmailContact;
use crate::state::use_email_contacts;
use dioxus::prelude::*;

/// System email detection banner component
#[component]
pub fn SystemEmailBanner(
    contact: EmailContact,
    checking: bool,
    is_system_detected: bool,
    converting: Signal<bool>,
) -> Element {
    let contacts_state = use_email_contacts();
    // Only show conversion suggestion if detected as system email AND not already a known system contact
    let show_convert_suggestion = is_system_detected && !contact.is_system;

    rsx! {
        if checking {
            div {
                class: "px-6 py-4 bg-blue-50 dark:bg-blue-900/20 border-b border-blue-200 dark:border-blue-800",
                p {
                    class: "text-sm text-blue-700 dark:text-blue-300",
                    "Checking if this is a system email..."
                }
            }
        }
        if show_convert_suggestion {
            div {
                class: "px-6 py-4 bg-amber-50 dark:bg-amber-900/20 border-b border-amber-200 dark:border-amber-800",
                div {
                    class: "flex items-start gap-3",
                    div {
                        class: "flex-1 min-w-0",
                        p {
                            class: "text-sm font-medium text-amber-800 dark:text-amber-200",
                            "System Email Detected"
                        }
                        p {
                            class: "text-sm text-amber-700 dark:text-amber-300 mt-1",
                            "This email matches a system email pattern. Convert it to a user-saved contact to prevent future detection."
                        }
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        class: "shrink-0",
                        disabled: converting() || *contacts_state.loading.read(),
                        onclick: move |_| {
                            *converting.write() = true;
                            contacts_state.convert_to_user_contact(contact.email.clone());
                        },
                        if converting() || *contacts_state.loading.read() {
                            "Converting..."
                        } else {
                            "Convert to User Contact"
                        }
                    }
                }
            }
        }
    }
}
