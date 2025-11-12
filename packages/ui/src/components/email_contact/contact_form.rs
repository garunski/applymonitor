//! Contact edit form component

use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::services::jobs_service::EmailContact;
use crate::state::use_email_contacts;
use dioxus::prelude::*;

/// Contact edit form component
#[component]
pub fn ContactForm(
    contact: EmailContact,
    name: Signal<String>,
    linkedin: Signal<String>,
    website: Signal<String>,
    saving: Signal<bool>,
    on_cancel: EventHandler,
    on_save: EventHandler,
) -> Element {
    let contacts_state = use_email_contacts();

    rsx! {
        div {
            class: "space-y-4",
            div {
                Label {
                    html_for: "contact-name",
                    "Name"
                }
                Input {
                    id: "contact-name",
                    r#type: "text",
                    value: "{name}",
                    oninput: move |e: Event<FormData>| *name.write() = e.value(),
                    placeholder: "Full name",
                }
            }
            div {
                Label {
                    html_for: "contact-linkedin",
                    "LinkedIn"
                }
                Input {
                    id: "contact-linkedin",
                    r#type: "url",
                    value: "{linkedin}",
                    oninput: move |e: Event<FormData>| *linkedin.write() = e.value(),
                    placeholder: "https://linkedin.com/in/...",
                }
            }
            div {
                Label {
                    html_for: "contact-website",
                    "Website"
                }
                Input {
                    id: "contact-website",
                    r#type: "url",
                    value: "{website}",
                    oninput: move |e: Event<FormData>| *website.write() = e.value(),
                    placeholder: "https://...",
                }
            }
            div {
                class: "flex justify-end gap-3 pt-4",
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        let name_opt = if name().trim().is_empty() {
                            None
                        } else {
                            Some(name().trim().to_string())
                        };
                        let linkedin_opt = if linkedin().trim().is_empty() {
                            None
                        } else {
                            Some(linkedin().trim().to_string())
                        };
                        let website_opt = if website().trim().is_empty() {
                            None
                        } else {
                            Some(website().trim().to_string())
                        };

                        let email = contact.email.clone();
                        let contacts_state_clone = contacts_state;
                        let mut saving_signal = saving;
                        let on_save_clone = on_save;
                        spawn(async move {
                            *saving_signal.write() = true;
                            contacts_state_clone.update_contact(
                                email,
                                name_opt,
                                linkedin_opt,
                                website_opt,
                            );
                            // update_contact spawns its own async task and updates the UI
                            // We just need to close the edit mode
                            *saving_signal.write() = false;
                            on_save_clone.call(());
                        });
                    },
                    disabled: saving(),
                    if saving() {
                        "Saving..."
                    } else {
                        "Save"
                    }
                }
            }
        }
    }
}
