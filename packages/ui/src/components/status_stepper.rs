//! Status stepper component

use crate::services::jobs_service::JobStatus;
use dioxus::prelude::*;

#[component]
pub fn StatusStepper(
    statuses: Vec<JobStatus>,
    current_status_id: Option<i32>,
    on_status_click: EventHandler<i32>,
) -> Element {
    let current_id = current_status_id.unwrap_or(0);
    let statuses_len = statuses.len();

    struct StepData {
        status: JobStatus,
        index: usize,
        is_completed: bool,
        is_current: bool,
        status_id: i32,
    }

    let steps: Vec<StepData> = statuses
        .iter()
        .enumerate()
        .map(|(idx, status)| StepData {
            status: status.clone(),
            index: idx + 1,
            is_completed: current_id > status.id,
            is_current: current_id == status.id,
            status_id: status.id,
        })
        .collect();

    rsx! {
        nav {
            aria_label: "Progress",
            ol {
                role: "list",
                class: "divide-y divide-gray-300 rounded-md border border-gray-300 md:flex md:divide-y-0 dark:divide-white/15 dark:border-white/15",
                for step in steps {
                    StatusStep {
                        status: step.status.clone(),
                        index: step.index,
                        is_completed: step.is_completed,
                        is_current: step.is_current,
                        total_count: statuses_len,
                        on_click: move |_| on_status_click.call(step.status_id),
                    }
                }
            }
        }
    }
}

#[component]
fn StatusStep(
    status: JobStatus,
    index: usize,
    is_completed: bool,
    is_current: bool,
    total_count: usize,
    on_click: EventHandler<()>,
) -> Element {
    let step_number = index.to_string();
    let is_last = index == total_count;

    rsx! {
        li {
            class: "relative md:flex md:flex-1",
            if is_completed {
                // Completed step
                button {
                    class: "group flex w-full items-center cursor-pointer",
                    onclick: move |_| on_click.call(()),
                    span {
                        class: "flex items-center px-6 py-4 text-sm font-medium",
                        span {
                            class: "flex size-10 shrink-0 items-center justify-center rounded-full bg-indigo-600 group-hover:bg-indigo-800 dark:bg-indigo-500 dark:group-hover:bg-indigo-400",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                "data-slot": "icon",
                                "aria-hidden": "true",
                                class: "size-6 text-white",
                                path {
                                    d: "M19.916 4.626a.75.75 0 0 1 .208 1.04l-9 13.5a.75.75 0 0 1-1.154.114l-6-6a.75.75 0 0 1 1.06-1.06l5.353 5.353 8.493-12.74a.75.75 0 0 1 1.04-.207Z",
                                    clip_rule: "evenodd",
                                    fill_rule: "evenodd",
                                }
                            }
                        }
                        span {
                            class: "ml-4 text-sm font-medium text-gray-900 dark:text-white",
                            {status.display_name.clone()}
                            if let Some(ref desc) = status.description {
                                div {
                                    class: "text-xs text-gray-500 dark:text-gray-400 mt-1",
                                    {desc.clone()}
                                }
                            }
                        }
                    }
                }
            } else if is_current {
                // Current step
                button {
                    class: "flex items-center px-6 py-4 text-sm font-medium cursor-pointer",
                    onclick: move |_| on_click.call(()),
                    span {
                        class: "flex size-10 shrink-0 items-center justify-center rounded-full border-2 border-indigo-600 dark:border-indigo-400",
                        span {
                            class: "text-indigo-600 dark:text-indigo-400",
                            {step_number}
                        }
                    }
                    span {
                        class: "ml-4 text-sm font-medium text-indigo-600 dark:text-indigo-400",
                        {status.display_name.clone()}
                        if let Some(ref desc) = status.description {
                            div {
                                class: "text-xs text-indigo-500 dark:text-indigo-300 mt-1",
                                {desc.clone()}
                            }
                        }
                    }
                }
            } else {
                // Upcoming step
                button {
                    class: "group flex items-center cursor-pointer",
                    onclick: move |_| on_click.call(()),
                    span {
                        class: "flex items-center px-6 py-4 text-sm font-medium",
                        span {
                            class: "flex size-10 shrink-0 items-center justify-center rounded-full border-2 border-gray-300 group-hover:border-gray-400 dark:border-white/15 dark:group-hover:border-white/25",
                            span {
                                class: "text-gray-500 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white",
                                {step_number}
                            }
                        }
                        span {
                            class: "ml-4 text-sm font-medium text-gray-500 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white",
                            {status.display_name.clone()}
                            if let Some(ref desc) = status.description {
                                div {
                                    class: "text-xs text-gray-400 group-hover:text-gray-600 dark:text-gray-500 dark:group-hover:text-gray-300 mt-1",
                                    {desc.clone()}
                                }
                            }
                        }
                    }
                }
            }
            // Arrow separator for md screens and up (not on last item)
            if !is_last {
                div {
                    "aria-hidden": "true",
                    class: "absolute right-0 top-0 hidden h-full w-5 md:block",
                    svg {
                        view_box: "0 0 22 80",
                        fill: "none",
                        preserve_aspect_ratio: "none",
                        class: "size-full text-gray-300 dark:text-white/15",
                        path {
                            d: "M0 -2L20 40L0 82",
                            stroke: "currentcolor",
                            vector_effect: "non-scaling-stroke",
                            stroke_linejoin: "round",
                        }
                    }
                }
            }
        }
    }
}
