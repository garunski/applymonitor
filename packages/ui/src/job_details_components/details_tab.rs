//! Details tab component

use crate::components::button::{Button, ButtonVariant};
use crate::components::select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue};
use crate::job_details_components::DescriptionField;
use crate::state::use_jobs;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsPencilSquare;
use dioxus_free_icons::Icon;

#[component]
pub fn DetailsTab(
    job_id: String,
    status_id: Option<i32>,
    created_at: Option<String>,
    updated_at: Option<String>,
    description: Option<String>,
    editing_status: Signal<bool>,
    editing_description: Signal<bool>,
    edit_status_value: Signal<String>,
    edit_description_value: Signal<String>,
) -> Element {
    let jobs_state = use_jobs();
    let job_id_status = job_id.clone();
    let job_id_desc = job_id.clone();
    let statuses = jobs_state.job_statuses.read().clone();
    let current_status = statuses.iter().find(|s| s.id == status_id.unwrap_or(0));

    rsx! {
        div {
            class: "space-y-4",
            // Status (editable)
            div {
                class: "flex justify-between items-center",
                span {
                    class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                    "Status"
                }
                if editing_status() {
                    div {
                        class: "flex items-center gap-2",
                        div {
                            class: "w-48",
                            Select::<i32> {
                                value: edit_status_value().parse::<i32>().ok(),
                                on_value_change: move |value: Option<i32>| {
                                    if let Some(v) = value {
                                        let mut val = edit_status_value;
                                        *val.write() = v.to_string();
                                    }
                                },
                                SelectTrigger {
                                    SelectValue {}
                                }
                                SelectList {
                                    for (idx, status) in statuses.iter().enumerate() {
                                        SelectOption::<i32> {
                                            value: status.id,
                                            text_value: status.display_name.clone(),
                                            index: use_signal(move || idx),
                                            {status.display_name.clone()}
                                        }
                                    }
                                }
                            }
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                if let Ok(status_id_val) = edit_status_value().parse::<i32>() {
                                    jobs_state.update_job_status(job_id_status.clone(), status_id_val);
                                }
                                let mut editing = editing_status;
                                *editing.write() = false;
                            },
                            "Save"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| {
                                let mut editing = editing_status;
                                *editing.write() = false;
                            },
                            "Cancel"
                        }
                    }
                } else {
                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "text-sm text-gray-900 dark:text-white",
                            {current_status.map(|s| s.display_name.clone()).unwrap_or_else(|| "Unknown".to_string())}
                        }
                        button {
                            class: "ml-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                            onclick: move |_| {
                                let mut val = edit_status_value;
                                *val.write() = status_id.map(|id| id.to_string()).unwrap_or_default();
                                let mut editing = editing_status;
                                *editing.write() = true;
                            },
                            Icon {
                                class: "h-3 w-3",
                                width: 12,
                                height: 12,
                                fill: "currentColor",
                                icon: BsPencilSquare,
                            }
                        }
                    }
                }
            }
            if let Some(ref created_at) = created_at {
                div {
                    class: "flex justify-between",
                    span {
                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                        "Created"
                    }
                    span {
                        class: "text-sm text-gray-900 dark:text-white",
                        {format_date(created_at)}
                    }
                }
            }
            if let Some(ref updated_at) = updated_at {
                div {
                    class: "flex justify-between",
                    span {
                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                        "Updated"
                    }
                    span {
                        class: "text-sm text-gray-900 dark:text-white",
                        {format_date(updated_at)}
                    }
                }
            }
        }

        DescriptionField {
            job_id: job_id_desc,
            description: description.clone(),
            editing: editing_description,
            edit_value: edit_description_value,
        }
    }
}

fn format_date(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt.format("%B %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}
