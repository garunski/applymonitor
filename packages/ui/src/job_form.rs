//! Job form component

use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue};
use crate::{
    services::jobs_service::{CreateJobRequest, Job, UpdateJobRequest},
    state::use_jobs,
};
use dioxus::prelude::*;

/// Job form component for creating and editing jobs
#[component]
pub fn JobForm(open: Signal<bool>, job: Option<Job>) -> Element {
    let jobs_state = use_jobs();
    let mut title = use_signal(|| job.as_ref().map(|j| j.title.clone()).unwrap_or_default());
    let mut company = use_signal(|| job.as_ref().map(|j| j.company.clone()).unwrap_or_default());
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
    let mut status_option = use_signal(|| Some(Some(status())));
    use_effect(move || {
        *status_option.write() = Some(Some(status()));
    });

    // Update form when job changes
    use_effect({
        let job_clone = job.clone();
        move || {
            if let Some(j) = &job_clone {
                *title.write() = j.title.clone();
                *company.write() = j.company.clone();
                *location.write() = j.location.clone().unwrap_or_default();
                *status.write() = j.status.clone();
            } else {
                *title.write() = String::new();
                *company.write() = String::new();
                *location.write() = String::new();
                *status.write() = "open".to_string();
            }
        }
    });

    let is_editing = job.is_some();
    let job_id = job.as_ref().and_then(|j| j.id);

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
                    class: "mt-4 space-y-4",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let title_val = title();
                        let company_val = company();
                        let location_val = if location().is_empty() { None } else { Some(location()) };
                        let status_val = status();

                        if title_val.is_empty() || company_val.is_empty() {
                            return;
                        }

                        if let Some(id) = job_id {
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
                            jobs_state.create_job(create_req);
                        }

                        *open.write() = false;
                    },
                    div {
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
                        Label {
                            html_for: "status",
                            "Status"
                        }
                        Select {
                            value: status_option,
                            on_value_change: move |new_status: Option<String>| {
                                if let Some(s) = new_status {
                                    *status.write() = s;
                                    *status_option.write() = Some(Some(status()));
                                }
                            },
                            SelectTrigger {
                                SelectValue {}
                            }
                            SelectList {
                                SelectOption::<String> {
                                    index: 0usize,
                                    value: "open".to_string(),
                                    text_value: "Open".to_string(),
                                    "Open"
                                }
                                SelectOption::<String> {
                                    index: 1usize,
                                    value: "applied".to_string(),
                                    text_value: "Applied".to_string(),
                                    "Applied"
                                }
                                SelectOption::<String> {
                                    index: 2usize,
                                    value: "interviewing".to_string(),
                                    text_value: "Interviewing".to_string(),
                                    "Interviewing"
                                }
                                SelectOption::<String> {
                                    index: 3usize,
                                    value: "offer".to_string(),
                                    text_value: "Offer".to_string(),
                                    "Offer"
                                }
                                SelectOption::<String> {
                                    index: 4usize,
                                    value: "rejected".to_string(),
                                    text_value: "Rejected".to_string(),
                                    "Rejected"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex justify-end space-x-2 pt-4",
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
