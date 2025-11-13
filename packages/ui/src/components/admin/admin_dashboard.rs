//! Admin dashboard component

use crate::services::admin_service::{AdminService, AdminStats};
use dioxus::prelude::*;

use super::{ai_metrics::AiMetrics, ai_prompts::AiPromptsEditor, users_list::UsersList};

#[component]
pub fn AdminDashboard() -> Element {
    let mut active_tab = use_signal(|| "overview".to_string());
    let mut stats = use_signal(|| None::<AdminStats>);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);

    use_effect(move || {
        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match AdminService::get_stats().await {
                Ok(admin_stats) => {
                    *stats.write() = Some(admin_stats);
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(format!("Failed to load stats: {:?}", e));
                }
            }

            *loading.write() = false;
        });
    });

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            div {
                class: "px-4 sm:px-6 lg:px-8 py-6",
                h1 {
                    class: "text-3xl font-bold text-gray-900 dark:text-gray-100 mb-6",
                    "Admin Dashboard"
                }

                // Tabs
                div {
                    class: "border-b border-gray-200 dark:border-gray-700 mb-6",
                    nav {
                        class: "-mb-px flex space-x-8",
                        AdminTabButton {
                            label: "Overview",
                            tab: "overview",
                            active: active_tab() == "overview",
                            onclick: move |_| *active_tab.write() = "overview".to_string(),
                        }
                        AdminTabButton {
                            label: "AI Prompts",
                            tab: "ai-prompts",
                            active: active_tab() == "ai-prompts",
                            onclick: move |_| *active_tab.write() = "ai-prompts".to_string(),
                        }
                        AdminTabButton {
                            label: "AI Metrics",
                            tab: "ai-metrics",
                            active: active_tab() == "ai-metrics",
                            onclick: move |_| *active_tab.write() = "ai-metrics".to_string(),
                        }
                    }
                }

                if let Some(err) = error() {
                    div {
                        class: "mb-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md",
                        p {
                            class: "text-sm text-red-800 dark:text-red-200",
                            {err}
                        }
                    }
                }

                match active_tab().as_str() {
                    "overview" => rsx! {
                        if loading() {
                            div {
                                class: "text-center py-8",
                                "Loading statistics..."
                            }
                        } else if let Some(stats_data) = stats() {
                            div {
                                class: "space-y-6",
                                div {
                                    class: "grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4",
                                    StatCard {
                                        title: "Total Users",
                                        value: stats_data.total_users.to_string(),
                                        color: "blue",
                                    }
                                    StatCard {
                                        title: "Enabled Users",
                                        value: stats_data.enabled_users.to_string(),
                                        color: "green",
                                    }
                                    StatCard {
                                        title: "Disabled Users",
                                        value: stats_data.disabled_users.to_string(),
                                        color: "red",
                                    }
                                    StatCard {
                                        title: "Admins",
                                        value: stats_data.admin_count.to_string(),
                                        color: "purple",
                                    }
                                }
                                UsersList {}
                            }
                        }
                    },
                    "ai-prompts" => rsx! {
                        AiPromptsEditor {}
                    },
                    "ai-metrics" => rsx! {
                        AiMetrics {}
                    },
                    _ => rsx! {
                        div { "Unknown tab" }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(title: String, value: String, color: String) -> Element {
    let bg_color = match color.as_str() {
        "blue" => "bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800",
        "green" => "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800",
        "red" => "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800",
        "purple" => "bg-purple-50 dark:bg-purple-900/20 border-purple-200 dark:border-purple-800",
        _ => "bg-gray-50 dark:bg-gray-800 border-gray-200 dark:border-gray-700",
    };

    let text_color = match color.as_str() {
        "blue" => "text-blue-600 dark:text-blue-400",
        "green" => "text-green-600 dark:text-green-400",
        "red" => "text-red-600 dark:text-red-400",
        "purple" => "text-purple-600 dark:text-purple-400",
        _ => "text-gray-600 dark:text-gray-400",
    };

    rsx! {
        div {
            class: "overflow-hidden rounded-lg border {bg_color} px-4 py-5 sm:p-6",
            dt {
                class: "truncate text-sm font-medium text-gray-500 dark:text-gray-400",
                {title}
            }
            dd {
                class: "mt-1 text-3xl font-semibold {text_color}",
                {value}
            }
        }
    }
}

#[component]
fn AdminTabButton(label: String, tab: String, active: bool, onclick: EventHandler) -> Element {
    let border_class = if active {
        "border-blue-500 text-blue-600 dark:text-blue-400"
    } else {
        "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600"
    };

    rsx! {
        button {
            class: "whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium {border_class}",
            onclick: move |_| onclick.call(()),
            {label}
        }
    }
}
