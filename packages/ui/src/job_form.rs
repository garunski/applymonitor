//! Job form component

use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::{
    services::jobs_service::{CreateJobRequest, Job, UpdateJobRequest},
    state::use_jobs,
};
use dioxus::prelude::*;

/// Job form component for creating and editing jobs
#[component]
pub fn JobForm(
    open: Signal<bool>,
    job: Option<Job>,
    prefill_title: Option<String>,
    prefill_company: Option<String>,
) -> Element {
    let jobs_state = use_jobs();
    let mut title = use_signal(|| {
        prefill_title
            .clone()
            .or_else(|| job.as_ref().map(|j| j.title.clone()))
            .unwrap_or_default()
    });
    let mut company = use_signal(|| {
        prefill_company
            .clone()
            .or_else(|| job.as_ref().map(|j| j.company.clone()))
            .unwrap_or_default()
    });
    let mut location = use_signal(|| {
        job.as_ref()
            .and_then(|j| j.location.clone())
            .unwrap_or_default()
    });
    let mut status = use_signal(|| {
        job.as_ref()
            .map(|j| j.status.clone())
            .unwrap_or_else(|| "open".to_string())
    });

    // Update form when job or prefill changes
    use_effect({
        let job_clone = job.clone();
        let prefill_title_clone = prefill_title.clone();
        let prefill_company_clone = prefill_company.clone();
        move || {
            if let Some(j) = &job_clone {
                *title.write() = j.title.clone();
                *company.write() = j.company.clone();
                *location.write() = j.location.clone().unwrap_or_default();
                *status.write() = j.status.clone();
            } else {
                *title.write() = prefill_title_clone.clone().unwrap_or_default();
                *company.write() = prefill_company_clone.clone().unwrap_or_default();
                *location.write() = String::new();
                *status.write() = "open".to_string();
            }
        }
    });

    let is_editing = job.is_some();
    let job_id_opt = job.as_ref().and_then(|j| j.id.clone());

    let mut open_option = use_signal(|| Some(*open.read()));
    use_effect(move || {
        *open_option.write() = Some(*open.read());
    });

    rsx! {
        DialogRoot {
            open: open_option,
            DialogContent {
                DialogTitle {
                    if is_editing {
                        "Edit Job"
                    } else {
                        "Create Job"
                    }
                }
                DialogDescription {
                    if is_editing {
                        "Update the job application details."
                    } else {
                        "Add a new job application to track."
                    }
                }
                form {
                    class: "space-y-4",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let title_val = title();
                        let company_val = company();
                        let location_val = if location().is_empty() { None } else { Some(location()) };
                        let status_val = status();

                        if title_val.is_empty() || company_val.is_empty() {
                            return;
                        }

                        if let Some(id) = job_id_opt.clone() {
                            let update_req = UpdateJobRequest {
                                title: title_val,
                                company: company_val,
                                location: location_val,
                                status: status_val,
                            };
                            jobs_state.update_job(id, update_req);
                        } else {
                            let create_req = CreateJobRequest {
                                title: title_val,
                                company: company_val,
                                location: location_val,
                                status: status_val,
                            };
                            let mut jobs_state_clone = jobs_state;
                            let mut open_signal = open;

                            spawn(async move {
                                match crate::services::jobs_service::JobsService::create_job(create_req).await {
                                    Ok(created_job) => {
                                        // Update jobs list
                                        let mut jobs_list = jobs_state_clone.jobs.read().clone();
                                        jobs_list.push(created_job.clone());
                                        *jobs_state_clone.jobs.write() = jobs_list;

                                        // Set created job ID for navigation
                                        if let Some(id) = created_job.id {
                                            jobs_state_clone.set_created_job_id(id);
                                        }

                                        *open_signal.write() = false;
                                    }
                                    Err(e) => {
                                        *jobs_state_clone.error.write() = Some(e);
                                    }
                                }
                            });
                            return;
                        }

                        *open.write() = false;
                    },
                    div {
                        class: "space-y-2",
                        Label {
                            html_for: "title",
                            "Title"
                        }
                        Input {
                            id: "title",
                            r#type: "text",
                            value: "{title}",
                            oninput: move |e: FormEvent| *title.write() = e.value(),
                            required: true,
                            placeholder: "e.g. Software Engineer",
                        }
                    }
                    div {
                        class: "space-y-2",
                        Label {
                            html_for: "company",
                            "Company"
                        }
                        Input {
                            id: "company",
                            r#type: "text",
                            value: "{company}",
                            oninput: move |e: FormEvent| *company.write() = e.value(),
                            required: true,
                            placeholder: "e.g. Acme Corp",
                        }
                    }
                    div {
                        class: "space-y-2",
                        Label {
                            html_for: "location",
                            "Location"
                        }
                        Input {
                            id: "location",
                            r#type: "text",
                            value: "{location}",
                            oninput: move |e: FormEvent| *location.write() = e.value(),
                            placeholder: "e.g. San Francisco, CA or Remote",
                        }
                    }
                    div {
                        class: "flex justify-end gap-3 pt-4",
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| *open.write() = false,
                            "Cancel"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            r#type: "submit",
                            if is_editing {
                                "Update"
                            } else {
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}
