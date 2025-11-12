//! Email contact slideout component for displaying and editing contact details

use crate::components::button::{Button, ButtonVariant};
use crate::components::email_contact::{ContactView, SlideoutHeader, SystemEmailBanner};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::hooks::use_system_email_detection;
use crate::state::use_email_contacts;
use dioxus::prelude::*;

/// Email contact slideout component
#[component]
pub fn EmailContactSlideout() -> Element {
    let contacts_state = use_email_contacts();
    let mut name = use_signal(String::new);
    let mut linkedin = use_signal(String::new);
    let mut website = use_signal(String::new);
    let mut is_editing = use_signal(|| false);
    let saving = use_signal(|| false);
    let converting = use_signal(|| false);

    let selected_contact_signal = contacts_state.selected_contact;

    // Use system email detection hook
    let detection = use_system_email_detection(selected_contact_signal);

    // Update form fields when contact changes
    use_effect({
        let contacts_state_effect = contacts_state;
        let mut name_signal = name;
        let mut linkedin_signal = linkedin;
        let mut website_signal = website;
        let mut editing = is_editing;
        let mut converting_signal = converting;
        move || {
            let contact = contacts_state_effect.selected_contact.read().clone();
            if let Some(ref c) = contact {
                *name_signal.write() = c.name.clone().unwrap_or_default();
                *linkedin_signal.write() = c.linkedin.clone().unwrap_or_default();
                *website_signal.write() = c.website.clone().unwrap_or_default();
                *editing.write() = false;
                *converting_signal.write() = false;
            }
        }
    });

    let current_contact = selected_contact_signal.read().clone();
    if let Some(contact) = current_contact {
        let display_name = contact
            .name
            .clone()
            .unwrap_or_else(|| contact.email.clone());

        rsx! {
            // Backdrop
            div {
                class: "fixed inset-0 z-50 bg-black/50 transition-opacity",
                onclick: move |_| {
                    contacts_state.clear_selected();
                },
            }

            // Slideout panel
            div {
                class: "fixed right-0 top-0 bottom-0 z-50 w-full sm:w-[500px] bg-white dark:bg-gray-900 shadow-xl transform transition-transform duration-300 ease-in-out overflow-y-auto",
                div {
                    class: "flex flex-col h-full",
                    SlideoutHeader {
                        contact: contact.clone(),
                        display_name: display_name.clone(),
                        on_close: move |_| {
                            contacts_state.clear_selected();
                        },
                    }

                    SystemEmailBanner {
                        contact: contact.clone(),
                        checking: *detection.checking.read(),
                        is_system_detected: *detection.is_system_detected.read(),
                        converting,
                    }

                    // Content - view or edit mode
                    div {
                        class: "flex-1 px-6 py-4 space-y-4",
                        if is_editing() {
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
                                        onclick: move |_| {
                                            *is_editing.write() = false;
                                            // Reset to current values from signal
                                            if let Some(current_contact) = selected_contact_signal.read().as_ref() {
                                                *name.write() = current_contact.name.clone().unwrap_or_default();
                                                *linkedin.write() = current_contact.linkedin.clone().unwrap_or_default();
                                                *website.write() = current_contact.website.clone().unwrap_or_default();
                                            }
                                        },
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
                                            let mut editing_signal = is_editing;
                                            spawn(async move {
                                                *saving_signal.write() = true;
                                                contacts_state_clone.update_contact(
                                                    email,
                                                    name_opt,
                                                    linkedin_opt,
                                                    website_opt,
                                                );
                                                *saving_signal.write() = false;
                                                *editing_signal.write() = false;
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
                        } else {
                            ContactView {
                                contact: contact.clone(),
                                on_edit: move |_| *is_editing.write() = true,
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}
