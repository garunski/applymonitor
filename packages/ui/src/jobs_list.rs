//! Jobs list component

use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::{job_form::JobForm, services::jobs_service::Job, state::use_jobs};
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
    let mut job_to_delete = use_signal(|| None::<String>);

    // Fetch jobs on mount
    use_effect(move || {
        jobs_state.fetch_jobs();
    });

    rsx! {
        div {
            class: "px-4 sm:px-6 lg:px-8 py-6",
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
                    class: "mt-6 text-center",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "Loading jobs..."
                    }
                }
            }

            // Error state
            if let Some(error) = jobs_state.error.read().as_ref() {
                div {
                    class: "mt-6 rounded-md bg-red-50 dark:bg-red-900/20 p-4",
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

            // Jobs list
            if !*jobs_state.loading.read() && jobs_state.error.read().is_none() {
                div {
                    class: "mt-6 flow-root",
                    ul {
                        role: "list",
                        class: "divide-y divide-gray-100 dark:divide-white/5",
                        for job in jobs_state.jobs.read().clone() {
                            li {
                                class: "flex justify-between gap-x-6 py-5",
                                // Left section: Avatar, title, company
                                div {
                                    class: "flex min-w-0 gap-x-4",
                                    // Company initial avatar
                                    div {
                                        class: "size-12 flex-none rounded-full bg-brand-100 dark:bg-brand-900 flex items-center justify-center dark:outline dark:outline-1 dark:-outline-offset-1 dark:outline-white/10",
                                        span {
                                            class: "text-brand-600 dark:text-brand-400 font-medium text-sm",
                                            {job.company.chars().next().unwrap_or('?').to_uppercase().collect::<String>()}
                                        }
                                    }
                                    // Text content
                                    div {
                                        class: "min-w-0 flex-auto",
                                        p {
                                            class: "text-sm/6 font-semibold text-gray-900 dark:text-white",
                                            a {
                                                href: "#",
                                                class: "hover:underline",
                                                {job.title.clone()}
                                            }
                                        }
                                        p {
                                            class: "mt-1 flex text-xs/5 text-gray-500 dark:text-gray-400",
                                            span {
                                                class: "truncate",
                                                {job.company.clone()}
                                                if let Some(ref loc) = job.location {
                                                    " • {loc}"
                                                }
                                            }
                                        }
                                    }
                                }
                                // Right section: Status, dropdown menu
                                div {
                                    class: "flex shrink-0 items-center gap-x-6",
                                    // Status badge (hidden on small screens)
                                    div {
                                        class: "hidden sm:flex sm:flex-col sm:items-end",
                                        p {
                                            class: "text-sm/6 text-gray-900 dark:text-white",
                                            {job.status.chars().next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default()}
                                            {job.status.chars().skip(1).collect::<String>()}
                                        }
                                    }
                                    // Dropdown menu
                                    DropdownMenu {
                                        DropdownMenuTrigger {
                                            button {
                                                class: "relative block text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white",
                                                span {
                                                    class: "absolute -inset-2.5",
                                                }
                                                span {
                                                    class: "sr-only",
                                                    "Open options"
                                                }
                                                svg {
                                                    view_box: "0 0 20 20",
                                                    fill: "currentColor",
                                                    "aria-hidden": "true",
                                                    class: "size-5",
                                                    path {
                                                        d: "M10 3a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM10 8.5a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM11.5 15.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z",
                                                    }
                                                }
                                            }
                                        }
                                        DropdownMenuContent {
                                            class: "right-0 w-32",
                                            DropdownMenuItem::<String> {
                                                index: use_signal(|| 0usize),
                                                value: "edit".to_string(),
                                                on_select: {
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
                                            DropdownMenuItem::<String> {
                                                index: use_signal(|| 1usize),
                                                value: "delete".to_string(),
                                                on_select: {
                                                    let job_id = job.id.clone();
                                                    let mut show_delete = show_delete_dialog;
                                                    let mut job_delete = job_to_delete;
                                                    move |_| {
                                                        if let Some(id) = job_id.clone() {
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
                prefill_title: None,
                prefill_company: None,
            }
            JobForm {
                open: show_edit_dialog,
                job: job_to_edit.read().clone(),
                prefill_title: None,
                prefill_company: None,
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
                                let id_opt = job_to_delete.read().clone();
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
