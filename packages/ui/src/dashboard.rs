//! Dashboard component for authenticated users

use crate::badge::Badge;
use crate::components::button::{Button, ButtonVariant};
use crate::job_form::JobForm;
use crate::services::jobs_service::Job;
use crate::state::use_jobs;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsBarChart, BsBriefcase, BsFileText, BsTrophy, BsXCircle},
    Icon,
};

/// Dashboard component showing job statistics and recent applications
#[component]
pub fn DashboardContent() -> Element {
    let jobs_state = use_jobs();
    let mut show_create_dialog = use_signal(|| false);

    // Fetch jobs on mount
    use_effect(move || {
        jobs_state.fetch_jobs();
    });

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
                    class: "mt-4 sm:mt-0",
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| *show_create_dialog.write() = true,
                        "Add Job"
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
                    // Total applications
                    div {
                        class: "bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg",
                        div {
                            class: "p-5",
                            div {
                                class: "flex items-center",
                                div {
                                    class: "flex-shrink-0",
                                    Icon {
                                        width: 24,
                                        height: 24,
                                        fill: "currentColor",
                                        icon: BsBarChart,
                                    }
                                }
                                div {
                                    class: "ml-5 w-0 flex-1",
                                    dl {
                                        dt {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                            "Total Applications"
                                        }
                                        dd {
                                            class: "mt-1 text-3xl font-semibold text-gray-900 dark:text-white",
                                            "{total_jobs}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Applied
                    div {
                        class: "bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg",
                        div {
                            class: "p-5",
                            div {
                                class: "flex items-center",
                                div {
                                    class: "flex-shrink-0",
                                    Icon {
                                        width: 24,
                                        height: 24,
                                        fill: "currentColor",
                                        icon: BsFileText,
                                    }
                                }
                                div {
                                    class: "ml-5 w-0 flex-1",
                                    dl {
                                        dt {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                            "Applied"
                                        }
                                        dd {
                                            class: "mt-1 text-3xl font-semibold text-blue-600 dark:text-blue-400",
                                            "{applied_count}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Interviewing
                    div {
                        class: "bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg",
                        div {
                            class: "p-5",
                            div {
                                class: "flex items-center",
                                div {
                                    class: "flex-shrink-0",
                                    Icon {
                                        width: 24,
                                        height: 24,
                                        fill: "currentColor",
                                        icon: BsBriefcase,
                                    }
                                }
                                div {
                                    class: "ml-5 w-0 flex-1",
                                    dl {
                                        dt {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                            "Interviewing"
                                        }
                                        dd {
                                            class: "mt-1 text-3xl font-semibold text-yellow-600 dark:text-yellow-400",
                                            "{interviewing_count}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Offers
                    div {
                        class: "bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg",
                        div {
                            class: "p-5",
                            div {
                                class: "flex items-center",
                                div {
                                    class: "flex-shrink-0",
                                    Icon {
                                        width: 24,
                                        height: 24,
                                        fill: "currentColor",
                                        icon: BsTrophy,
                                    }
                                }
                                div {
                                    class: "ml-5 w-0 flex-1",
                                    dl {
                                        dt {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                            "Offers"
                                        }
                                        dd {
                                            class: "mt-1 text-3xl font-semibold text-green-600 dark:text-green-400",
                                            "{offer_count}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Rejected
                    div {
                        class: "bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg",
                        div {
                            class: "p-5",
                            div {
                                class: "flex items-center",
                                div {
                                    class: "flex-shrink-0",
                                    Icon {
                                        width: 24,
                                        height: 24,
                                        fill: "currentColor",
                                        icon: BsXCircle,
                                    }
                                }
                                div {
                                    class: "ml-5 w-0 flex-1",
                                    dl {
                                        dt {
                                            class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                            "Rejected"
                                        }
                                        dd {
                                            class: "mt-1 text-3xl font-semibold text-red-600 dark:text-red-400",
                                            "{rejected_count}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Recent applications
                div {
                    class: "bg-white dark:bg-gray-800 shadow rounded-lg",
                    div {
                        class: "px-4 py-5 sm:px-6",
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-white mb-4",
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
                            div {
                                class: "flow-root",
                                ul {
                                    class: "-my-5 divide-y divide-gray-200 dark:divide-gray-700",
                                    for job in recent_jobs.iter() {
                                        li {
                                            class: "py-4",
                                            div {
                                                class: "flex items-center space-x-4",
                                                div {
                                                    class: "flex-shrink-0",
                                                    div {
                                                        class: "h-10 w-10 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center",
                                                        span {
                                                            class: "text-blue-600 dark:text-blue-400 font-medium",
                                                            {job.company.chars().next().unwrap_or('?').to_uppercase().collect::<String>()}
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "flex-1 min-w-0",
                                                    p {
                                                        class: "text-sm font-medium text-gray-900 dark:text-white truncate",
                                                        {job.title.clone()}
                                                    }
                                                    p {
                                                        class: "text-sm text-gray-500 dark:text-gray-400 truncate",
                                                        {job.company.clone()}
                                                        if let Some(loc) = &job.location {
                                                            " • {loc}"
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "flex-shrink-0",
                                                    Badge { status: job.status.clone() }
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

            // Create job dialog
            JobForm {
                open: show_create_dialog,
                job: None,
            }
        }
    }
}
