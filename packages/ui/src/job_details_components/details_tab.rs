//! Details tab component

use crate::job_details_components::DescriptionField;
use dioxus::prelude::*;

#[component]
pub fn DetailsTab(
    job_id: String,
    created_at: Option<String>,
    updated_at: Option<String>,
    description: Option<String>,
    editing_description: Signal<bool>,
    edit_description_value: Signal<String>,
) -> Element {
    let job_id_desc = job_id.clone();

    rsx! {
        div {
            class: "space-y-4",
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
