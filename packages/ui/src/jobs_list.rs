//! Jobs list component

use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::button::{Button, ButtonVariant};
use crate::{
    badge::Badge,
    job_form::JobForm,
    services::jobs_service::Job,
    state::use_jobs,
    table::{Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow},
};
use dioxus::prelude::*;
use std::rc::Rc;

/// Jobs list component
#[component]
pub fn JobsList() -> Element {
    let jobs_state = use_jobs();
    #[allow(unused_mut)]
    let mut show_create_dialog = use_signal(|| false);
    #[allow(unused_mut)]
    let mut show_edit_dialog = use_signal(|| false);
    #[allow(unused_mut)]
    let mut job_to_edit = use_signal(|| None::<Job>);
    let mut show_delete_dialog = use_signal(|| Some(false));
    let mut job_to_delete = use_signal(|| None::<i64>);

    // Fetch jobs on mount
    use_effect(move || {
        jobs_state.fetch_jobs();
    });

    rsx! {
        div {
            class: "px-4 sm:px-6 lg:px-8 py-8",
            div {
                class: "sm:flex sm:items-center",
                div {
                    class: "sm:flex-auto",
                    h1 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white",
                        "Jobs"
                    }
                    p {
                        class: "mt-2 text-sm text-gray-700 dark:text-gray-300",
                        "A list of all your job applications."
                    }
                }
                div {
                    class: "mt-4 sm:ml-16 sm:mt-0 sm:flex-none",
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| *show_create_dialog.write() = true,
                        "Add job"
                    }
                }
            }

            // Loading state
            if *jobs_state.loading.read() {
                div {
                    class: "mt-8 text-center",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "Loading jobs..."
                    }
                }
            }

            // Error state
            if let Some(error) = jobs_state.error.read().as_ref() {
                div {
                    class: "mt-8 rounded-md bg-red-50 dark:bg-red-900/20 p-4",
                    div {
                        class: "flex",
                        div {
                            class: "ml-3",
                            h3 {
                                class: "text-sm font-medium text-red-800 dark:text-red-200",
                                "Error loading jobs"
                            }
                            div {
                                class: "mt-2 text-sm text-red-700 dark:text-red-300",
                                "{error}"
                            }
                        }
                    }
                }
            }

            // Jobs table
            if !*jobs_state.loading.read() && jobs_state.error.read().is_none() {
                div {
                    class: "mt-8 flow-root",
                    Table {
                        TableHeader {
                            TableRow {
                                TableHeaderCell { "Title" }
                                TableHeaderCell { "Company" }
                                TableHeaderCell { "Location" }
                                TableHeaderCell { "Status" }
                                TableHeaderCell { "Actions" }
                            }
                        }
                        TableBody {
                            for job in jobs_state.jobs.read().clone() {
                                TableRow {
                                    TableCell {
                                        div {
                                            class: "font-medium text-gray-900 dark:text-white",
                                            {job.title.clone()}
                                        }
                                    }
                                    TableCell {
                                        {job.company.clone()}
                                    }
                                    TableCell {
                                        {job.location.clone().unwrap_or_default()}
                                    }
                                    TableCell {
                                        Badge { status: job.status.clone() }
                                    }
                                    TableCell {
                                        div {
                                            class: "flex space-x-2",
                                            Button {
                                                variant: ButtonVariant::Secondary,
                                                onclick: {
                                                    let job_rc = Rc::new(job.clone());
                                                    let mut show_edit = show_edit_dialog;
                                                    let mut job_edit = job_to_edit;
                                                    move |_| {
                                                        *job_edit.write() = Some((*job_rc).clone());
                                                        *show_edit.write() = true;
                                                    }
                                                },
                                                "Edit"
                                            }
                                            Button {
                                                variant: ButtonVariant::Destructive,
                                                onclick: {
                                                    let job_id = job.id;
                                                    let mut show_delete = show_delete_dialog;
                                                    let mut job_delete = job_to_delete;
                                                    move |_| {
                                                        if let Some(id) = job_id {
                                                            *job_delete.write() = Some(id);
                                                            *show_delete.write() = Some(true);
                                                        }
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Create/Edit job dialog
            JobForm {
                open: show_create_dialog,
                job: None,
            }
            JobForm {
                open: show_edit_dialog,
                job: job_to_edit.read().clone(),
            }

            // Delete confirmation dialog
            AlertDialogRoot {
                open: show_delete_dialog,
                AlertDialogContent {
                    AlertDialogTitle {
                        "Delete Job"
                    }
                    AlertDialogDescription {
                        "Are you sure you want to delete this job? This action cannot be undone."
                    }
                    AlertDialogActions {
                        AlertDialogCancel {
                            on_click: move |_| {
                                *show_delete_dialog.write() = Some(false);
                                *job_to_delete.write() = None;
                            },
                            "Cancel"
                        }
                        AlertDialogAction {
                            on_click: move |_| {
                                let id_opt = *job_to_delete.read();
                                if let Some(id) = id_opt {
                                    jobs_state.delete_job(id);
                                    *show_delete_dialog.write() = Some(false);
                                    *job_to_delete.write() = None;
                                }
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
