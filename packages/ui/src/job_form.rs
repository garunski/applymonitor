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
    gmail_id: Option<String>,
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
    let mut status_id = use_signal(|| job.as_ref().and_then(|j| j.status_id).unwrap_or(100));

    // Fetch statuses on mount
    use_effect({
        let jobs_state_statuses = jobs_state;
        move || {
            jobs_state_statuses.fetch_job_statuses();
        }
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
                *status_id.write() = j.status_id.unwrap_or(100);
            } else {
                *title.write() = prefill_title_clone.clone().unwrap_or_default();
                *company.write() = prefill_company_clone.clone().unwrap_or_default();
                *location.write() = String::new();
                *status_id.write() = 100;
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
                        let status_id_val = status_id();

                        if title_val.is_empty() || company_val.is_empty() {
                            return;
                        }

                        if let Some(id) = job_id_opt.clone() {
                            let update_req = UpdateJobRequest {
                                title: title_val,
                                company: company_val,
                                location: location_val,
                                status_id: Some(status_id_val),
                            };
                            jobs_state.update_job(id, update_req);
                        } else {
                            let gmail_id_clone = gmail_id.clone();
                            let mut jobs_state_clone = jobs_state;
                            let mut open_signal = open;

                            spawn(async move {
                                // If gmail_id is provided, use assign_email_to_job endpoint
                                // which creates job AND links email in one call
                                if let Some(gmail_id) = gmail_id_clone {
                                    let create_req = crate::services::emails_service::CreateJobFromEmail {
                                        title: title_val,
                                        company: company_val,
                                        location: location_val,
                                        status_id: Some(status_id_val),
                                    };
                                    let assign_req = crate::services::emails_service::AssignJobRequest {
                                        job_id: None,
                                        create_job: Some(create_req),
                                    };

                                    match crate::services::emails_service::EmailsService::assign_email_to_job(gmail_id, assign_req).await {
                                        Ok(response) => {
                                            // Fetch the created job to add to list
                                            if let Ok(created_job) = crate::services::jobs_service::JobsService::fetch_job(response.job_id.clone()).await {
                                                let mut jobs_list = jobs_state_clone.jobs.read().clone();
                                                jobs_list.push(created_job.clone());
                                                *jobs_state_clone.jobs.write() = jobs_list;
                                            }

                                            // Set created job ID for navigation
                                            jobs_state_clone.set_created_job_id(response.job_id);
                                            *open_signal.write() = false;
                                        }
                                        Err(e) => {
                                            *jobs_state_clone.error.write() = Some(crate::services::error::ServiceError::Server(500, format!("Failed to create job: {:?}", e)));
                                        }
                                    }
                                } else {
                                    // No gmail_id, use regular create_job endpoint
                                    let create_req = CreateJobRequest {
                                        title: title_val,
                                        company: company_val,
                                        location: location_val,
                                        status_id: Some(status_id_val),
                                    };

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
