//! Details tab component

use crate::job_details_components::DescriptionField;
use crate::state::use_auth;
use crate::utils::format_date;
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
    let auth = use_auth();
    let job_id_desc = job_id.clone();
    let user = auth.user.read();
    let timezone = user.as_ref().and_then(|u| u.timezone.as_deref());
    let formatted_created = created_at.as_ref().map(|d| format_date(d, timezone));
    let formatted_updated = updated_at.as_ref().map(|d| format_date(d, timezone));

    rsx! {
        div {
            class: "space-y-4",
            if let Some(ref formatted_date) = formatted_created {
                div {
                    class: "flex justify-between",
                    span {
                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                        "Created"
                    }
                    span {
                        class: "text-sm text-gray-900 dark:text-white",
                        {formatted_date.clone()}
                    }
                }
            }
            if let Some(ref formatted_date) = formatted_updated {
                div {
                    class: "flex justify-between",
                    span {
                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                        "Updated"
                    }
                    span {
                        class: "text-sm text-gray-900 dark:text-white",
                        {formatted_date.clone()}
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
