//! Job details header with editable title, company, location

use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::status_stepper::StatusStepper;
use crate::job_details_components::{CompanyField, LocationField};
use crate::services::jobs_service::JobStatus;
use crate::state::use_jobs;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsPencilSquare;
use dioxus_free_icons::Icon;

#[component]
pub fn JobDetailsHeader(
    job_id: String,
    title: String,
    company: String,
    location: Option<String>,
    status_id: Option<i32>,
    statuses: Vec<JobStatus>,
    on_status_click: EventHandler<i32>,
    editing_title: Signal<bool>,
    editing_company: Signal<bool>,
    editing_location: Signal<bool>,
    edit_title_value: Signal<String>,
    edit_company_value: Signal<String>,
    edit_location_value: Signal<String>,
) -> Element {
    let jobs_state = use_jobs();
    let job_id_title = job_id.clone();
    let job_id_company = job_id.clone();
    let job_id_location = job_id.clone();

    rsx! {
        div {
            class: "mb-6",
            // Status stepper
            div {
                class: "mb-6",
                StatusStepper {
                    statuses: statuses.clone(),
                    current_status_id: status_id,
                    on_status_click: move |id| on_status_click.call(id),
                }
            }
            // Title (editable)
            div {
                class: "flex items-center gap-2",
                if editing_title() {
                    div {
                        class: "flex-1 flex items-center gap-2",
                        Input {
                            id: "edit-title",
                            r#type: "text",
                            value: edit_title_value(),
                            oninput: move |e: Event<FormData>| {
                                let mut val = edit_title_value;
                                *val.write() = e.value();
                            },
                            class: "flex-1",
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                let title_val = edit_title_value();
                                if !title_val.trim().is_empty() {
                                    jobs_state.update_job_title(job_id_title.clone(), title_val);
                                }
                                let mut editing = editing_title;
                                *editing.write() = false;
                            },
                            "Save"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| {
                                let mut editing = editing_title;
                                *editing.write() = false;
                            },
                            "Cancel"
                        }
                    }
                } else {
                    h1 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white flex items-center gap-2",
                        {title.clone()}
                        button {
                            class: "ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                            onclick: move |_| {
                                let mut val = edit_title_value;
                                *val.write() = title.clone();
                                let mut editing = editing_title;
                                *editing.write() = true;
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
            // Company and location (editable)
            div {
                class: "mt-1 flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400",
                CompanyField {
                    job_id: job_id_company,
                    company: company.clone(),
                    editing: editing_company,
                    edit_value: edit_company_value,
                }
                LocationField {
                    job_id: job_id_location,
                    location: location.clone(),
                    editing: editing_location,
                    edit_value: edit_location_value,
                }
            }
        }
    }
}
