//! Email contact slideout component for displaying and editing contact details

use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::state::use_email_contacts;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsX;
use dioxus_free_icons::Icon;

/// Email contact slideout component
#[component]
pub fn EmailContactSlideout() -> Element {
    let contacts_state = use_email_contacts();
    let mut name = use_signal(String::new);
    let mut linkedin = use_signal(String::new);
    let mut website = use_signal(String::new);
    let mut is_editing = use_signal(|| false);
    let saving = use_signal(|| false);

    let selected_contact = contacts_state.selected_contact.read().clone();

    // Update form fields when contact changes
    use_effect({
        let contact = selected_contact.clone();
        let mut name_signal = name;
        let mut linkedin_signal = linkedin;
        let mut website_signal = website;
        let mut editing = is_editing;
        move || {
            if let Some(ref c) = contact {
                *name_signal.write() = c.name.clone().unwrap_or_default();
                *linkedin_signal.write() = c.linkedin.clone().unwrap_or_default();
                *website_signal.write() = c.website.clone().unwrap_or_default();
                *editing.write() = false;
            }
        }
    });

    if let Some(contact) = selected_contact {
        let is_system = contact.is_system;
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
                    // Header
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
                                onclick: move |_| {
                                    contacts_state.clear_selected();
                                },
                                Icon {
                                    class: "w-5 h-5",
                                    width: 20,
                                    height: 20,
                                    fill: "currentColor",
                                    icon: BsX,
                                }
                            }
                        }

                        // System email badge
                        if is_system {
                            div {
                                class: "mb-4",
                                span {
                                    class: "inline-flex items-center rounded-md bg-indigo-50 dark:bg-indigo-900/30 px-3 py-1 text-sm font-medium text-indigo-700 dark:text-indigo-300 ring-1 ring-inset ring-indigo-700/10 dark:ring-indigo-300/20",
                                    "System Email"
                                }
                            }
                        }
                    }

                    // Content
                    div {
                        class: "flex-1 px-6 py-4 space-y-6",
                        if is_editing() {
                            // Edit form
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
                                        onclick: {
                                            let mut editing = is_editing;
                                            let contact_clone = contact.clone();
                                            let mut name_signal = name;
                                            let mut linkedin_signal = linkedin;
                                            let mut website_signal = website;
                                            move |_| {
                                                *editing.write() = false;
                                                // Reset to original values
                                                *name_signal.write() = contact_clone.name.clone().unwrap_or_default();
                                                *linkedin_signal.write() = contact_clone.linkedin.clone().unwrap_or_default();
                                                *website_signal.write() = contact_clone.website.clone().unwrap_or_default();
                                            }
                                        },
                                        "Cancel"
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        onclick: {
                                            let email = contact.email.clone();
                                            let name_value = name;
                                            let linkedin_value = linkedin;
                                            let website_value = website;
                                            let saving_signal = saving;
                                            let editing_signal = is_editing;
                                            let contacts_state_clone = contacts_state;
                                            move |_| {
                                                let name_opt = if name_value().trim().is_empty() {
                                                    None
                                                } else {
                                                    Some(name_value().trim().to_string())
                                                };
                                                let linkedin_opt = if linkedin_value().trim().is_empty() {
                                                    None
                                                } else {
                                                    Some(linkedin_value().trim().to_string())
                                                };
                                                let website_opt = if website_value().trim().is_empty() {
                                                    None
                                                } else {
                                                    Some(website_value().trim().to_string())
                                                };

                                                let email_clone = email.clone();
                                                let mut saving_clone = saving_signal;
                                                let mut editing_clone = editing_signal;

                                                spawn(async move {
                                                    *saving_clone.write() = true;
                                                    contacts_state_clone.update_contact(
                                                        email_clone,
                                                        name_opt,
                                                        linkedin_opt,
                                                        website_opt,
                                                    );
                                                    *saving_clone.write() = false;
                                                    *editing_clone.write() = false;
                                                });
                                            }
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
                            // View mode
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
                                        onclick: move |_| *is_editing.write() = true,
                                        "Edit"
                                    }
                                }
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
