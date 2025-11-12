//! Dashboard component for authenticated users

use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::statistics_card::StatisticsCard;
use crate::job_form::JobForm;
use crate::services::gmail_scanner_service::{GmailScannerService, GmailStatus};
use crate::services::jobs_service::Job;
use crate::state::use_jobs;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsBarChart, BsBriefcase, BsEnvelope, BsFileText, BsTrophy, BsXCircle},
    Icon,
};
use std::rc::Rc;

/// Dashboard component showing job statistics and recent applications
#[component]
pub fn DashboardContent() -> Element {
    let jobs_state = use_jobs();
    let mut show_create_dialog = use_signal(|| false);
    #[allow(unused_mut)]
    let mut show_edit_dialog = use_signal(|| false);
    #[allow(unused_mut)]
    let mut job_to_edit = use_signal(|| None::<Job>);
    let mut show_delete_dialog = use_signal(|| Some(false));
    let mut job_to_delete = use_signal(|| None::<String>);

    // Gmail scanning state
    let scanning = use_signal(|| false);
    let scan_error = use_signal(|| None::<String>);
    let scan_success = use_signal(|| None::<usize>);
    let gmail_status = use_signal(|| None::<GmailStatus>);

    // Fetch jobs on mount
    use_effect(move || {
        jobs_state.fetch_jobs();
    });

    // Fetch Gmail connection status on mount
    use_effect(move || {
        let mut status = gmail_status;
        spawn(async move {
            match GmailScannerService::get_scan_status().await {
                Ok(status_data) => {
                    *status.write() = Some(status_data);
                }
                Err(_) => {
                    *status.write() = None;
                }
            }
        });
    });

    let gmail_connected = gmail_status
        .read()
        .as_ref()
        .map(|s| s.connected)
        .unwrap_or(false);

    let handle_scan = {
        let mut scanning_state = scanning;
        let mut error_state = scan_error;
        let mut success_state = scan_success;

        move |_| {
            if *scanning_state.read() {
                return;
            }

            *scanning_state.write() = true;
            *error_state.write() = None;
            *success_state.write() = None;

            let mut scanning_inner = scanning_state;
            let mut error_inner = error_state;
            let mut success_inner = success_state;

            spawn(async move {
                match GmailScannerService::scan_emails(None, None).await {
                    Ok(response) => {
                        *success_inner.write() = Some(response.emails_found);
                        *error_inner.write() = None;
                        // Refresh jobs after successful scan
                        jobs_state.fetch_jobs();
                    }
                    Err(e) => {
                        *error_inner.write() = Some(format!("{}", e));
                        *success_inner.write() = None;
                    }
                }
                *scanning_inner.write() = false;
            });
        }
    };

    let jobs = jobs_state.jobs.read().clone();
    let total_jobs = jobs.len();
    let applied_count = jobs.iter().filter(|j| j.status == "applied").count();
    let interviewing_count = jobs.iter().filter(|j| j.status == "interviewing").count();
    let offer_count = jobs.iter().filter(|j| j.status == "offer").count();
    let rejected_count = jobs.iter().filter(|j| j.status == "rejected").count();
    let recent_jobs: Vec<Job> = jobs.iter().take(5).cloned().collect();

    rsx! {
        div {
            class: "px-4 sm:px-6 lg:px-8 py-6",
            // Header
            div {
                class: "sm:flex sm:items-center sm:justify-between mb-6",
                div {
                    h1 {
                        class: "text-3xl font-bold text-gray-900 dark:text-white",
                        "Dashboard"
                    }
                    p {
                        class: "mt-2 text-sm text-gray-600 dark:text-gray-400",
                        "Overview of your job applications"
                    }
                }
                div {
                    class: "mt-4 sm:mt-0 flex gap-3",
                    if gmail_connected {
                        Button {
                            variant: ButtonVariant::Ghost,
                            class: if *scanning.read() {
                                "flex items-center justify-center gap-2 opacity-50 cursor-not-allowed"
                            } else {
                                "flex items-center justify-center gap-2"
                            },
                            onclick: handle_scan,
                            Icon {
                                class: "w-5 h-5",
                                width: 20,
                                height: 20,
                                fill: "currentColor",
                                icon: BsEnvelope,
                            }
                            if *scanning.read() {
                                "Scanning..."
                            } else {
                                "Scan Gmail"
                            }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| *show_create_dialog.write() = true,
                        "Add Job"
                    }
                }
            }

            // Gmail scan messages
            if let Some(error) = scan_error.read().as_ref() {
                div {
                    class: "mb-6 rounded-md bg-red-50 dark:bg-red-900/20 p-4",
                    div {
                        class: "flex",
                        div {
                            class: "ml-3",
                            h3 {
                                class: "text-sm font-medium text-red-800 dark:text-red-200",
                                "Scan Error"
                            }
                            div {
                                class: "mt-2 text-sm text-red-700 dark:text-red-300",
                                {error.clone()}
                            }
                        }
                    }
                }
            }
            if let Some(count) = scan_success.read().as_ref() {
                div {
                    class: "mb-6 rounded-md bg-green-50 dark:bg-green-900/20 p-4",
                    div {
                        class: "flex",
                        div {
                            class: "ml-3",
                            h3 {
                                class: "text-sm font-medium text-green-800 dark:text-green-200",
                                "Scan Complete"
                            }
                            div {
                                class: "mt-2 text-sm text-green-700 dark:text-green-300",
                                {
                                    let email_text = if *count == 1 { "email" } else { "emails" };
                                    format!("Found {} {} in your Gmail.", count, email_text)
                                }
                            }
                        }
                    }
                }
            }

            // Loading state
            if *jobs_state.loading.read() {
                div {
                    class: "text-center py-12",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "Loading dashboard..."
                    }
                }
            }

            // Error state
            if let Some(error) = jobs_state.error.read().as_ref() {
                div {
                    class: "rounded-md bg-red-50 dark:bg-red-900/20 p-4 mb-6",
                    div {
                        class: "flex",
                        div {
                            class: "ml-3",
                            h3 {
                                class: "text-sm font-medium text-red-800 dark:text-red-200",
                                "Error loading dashboard"
                            }
                            div {
                                class: "mt-2 text-sm text-red-700 dark:text-red-300",
                                "{error}"
                            }
                        }
                    }
                }
            }

            // Dashboard content
            if !*jobs_state.loading.read() && jobs_state.error.read().is_none() {
                // Statistics cards
                div {
                    class: "grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-5 mb-6",
                    StatisticsCard {
                        icon: rsx! {
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsBarChart,
                            }
                        },
                        value: total_jobs,
                        label: "Total Applications".to_string(),
                    }
                    StatisticsCard {
                        icon: rsx! {
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsFileText,
                            }
                        },
                        value: applied_count,
                        label: "Applied".to_string(),
                        value_color: "text-brand-600 dark:text-brand-400".to_string(),
                    }
                    StatisticsCard {
                        icon: rsx! {
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsBriefcase,
                            }
                        },
                        value: interviewing_count,
                        label: "Interviewing".to_string(),
                        value_color: "text-yellow-600 dark:text-yellow-400".to_string(),
                    }
                    StatisticsCard {
                        icon: rsx! {
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsTrophy,
                            }
                        },
                        value: offer_count,
                        label: "Offers".to_string(),
                        value_color: "text-green-600 dark:text-green-400".to_string(),
                    }
                    StatisticsCard {
                        icon: rsx! {
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsXCircle,
                            }
                        },
                        value: rejected_count,
                        label: "Rejected".to_string(),
                        value_color: "text-red-600 dark:text-red-400".to_string(),
                    }
                }

                // Recent applications
                div {
                    class: "mt-6 flow-root",
                    h3 {
                        class: "text-lg font-medium text-gray-900 dark:text-white mb-6",
                        "Recent Applications"
                    }
                    if recent_jobs.is_empty() {
                        div {
                            class: "text-center py-8",
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "No applications yet. Add your first job to get started!"
                            }
                        }
                    } else {
                        ul {
                            role: "list",
                            class: "divide-y divide-gray-100 dark:divide-white/5",
                            for job in recent_jobs.iter() {
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
