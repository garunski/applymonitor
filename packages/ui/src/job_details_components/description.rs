//! Description editing component

use crate::components::button::{Button, ButtonVariant};
use crate::state::use_jobs;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsPencilSquare;
use dioxus_free_icons::Icon;

#[component]
pub fn DescriptionField(
    job_id: String,
    description: Option<String>,
    editing: Signal<bool>,
    edit_value: Signal<String>,
) -> Element {
    let jobs_state = use_jobs();
    let description_clone = description.clone();

    rsx! {
        div {
            class: "mt-4 pt-4 border-t border-gray-200 dark:border-gray-700",
            label {
                class: "text-sm font-medium text-gray-500 dark:text-gray-400 mb-2 block",
                "Description"
            }
            if editing() {
                div {
                    class: "space-y-2",
                    div {
                        class: "overflow-hidden rounded-lg outline outline-1 -outline-offset-1 outline-gray-300 focus-within:outline focus-within:outline-2 focus-within:-outline-offset-2 focus-within:outline-indigo-600 dark:bg-white/5 dark:outline-white/10 dark:focus-within:outline-indigo-500",
                        textarea {
                            id: "edit-description",
                            class: "block w-full resize-none bg-transparent px-3 py-1.5 text-base text-gray-900 placeholder:text-gray-400 focus:outline focus:outline-0 sm:text-sm/6 dark:text-white dark:placeholder:text-gray-500",
                            rows: "6",
                            value: edit_value(),
                            oninput: move |e: Event<FormData>| {
                                let mut val = edit_value;
                                *val.write() = e.value();
                            },
                            placeholder: "Job description (optional)",
                        }
                    }
                    div {
                        class: "flex gap-2",
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                let description_val = edit_value();
                                let desc_opt = if description_val.trim().is_empty() {
                                    None
                                } else {
                                    Some(description_val.trim().to_string())
                                };
                                jobs_state.update_job_description(job_id.clone(), desc_opt);
                                let mut editing_signal = editing;
                                *editing_signal.write() = false;
                            },
                            "Save"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| {
                                let mut editing_signal = editing;
                                *editing_signal.write() = false;
                            },
                            "Cancel"
                        }
                    }
                }
            } else {
                if let Some(ref desc) = description_clone {
                    if desc.is_empty() {
                        div {
                            class: "flex items-center justify-between",
                            p {
                                class: "text-sm text-gray-400 dark:text-gray-500 italic",
                                "No description"
                            }
                            button {
                                class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                                onclick: move |_| {
                                    let mut val = edit_value;
                                    *val.write() = String::new();
                                    let mut editing_signal = editing;
                                    *editing_signal.write() = true;
                                },
                                Icon {
                                    class: "h-4 w-4",
                                    width: 16,
                                    height: 16,
                                    fill: "currentColor",
                                    icon: BsPencilSquare,
                                }
                            }
                        }
                    } else {
                        div {
                            class: "flex items-start justify-between gap-2",
                            p {
                                class: "text-sm text-gray-900 dark:text-white whitespace-pre-wrap flex-1",
                                {desc.clone()}
                            }
                            button {
                                class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                                onclick: {
                                    let desc_value = desc.clone();
                                    move |_| {
                                        let mut val = edit_value;
                                        *val.write() = desc_value.clone();
                                        let mut editing_signal = editing;
                                        *editing_signal.write() = true;
                                    }
                                },
                                Icon {
                                    class: "h-4 w-4",
                                    width: 16,
                                    height: 16,
                                    fill: "currentColor",
                                    icon: BsPencilSquare,
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "flex items-center justify-between",
                        p {
                            class: "text-sm text-gray-400 dark:text-gray-500 italic",
                            "No description"
                        }
                        button {
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                            onclick: move |_| {
                                let mut val = edit_value;
                                *val.write() = String::new();
                                let mut editing_signal = editing;
                                *editing_signal.write() = true;
                            },
                            Icon {
                                class: "h-4 w-4",
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: BsPencilSquare,
                            }
                        }
                    }
                }
            }
        }
    }
}
