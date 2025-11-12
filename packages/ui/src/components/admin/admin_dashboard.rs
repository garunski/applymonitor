//! Admin dashboard component

use crate::services::admin_service::{AdminService, AdminStats};
use dioxus::prelude::*;

use super::users_list::UsersList;

#[component]
pub fn AdminDashboard() -> Element {
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

                if let Some(err) = error() {
                    div {
                        class: "mb-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md",
                        p {
                            class: "text-sm text-red-800 dark:text-red-200",
                            {err}
                        }
                    }
                }

                if loading() {
                    div {
                        class: "text-center py-8",
                        "Loading statistics..."
                    }
                } else if let Some(stats_data) = stats() {
                    div {
                        class: "grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4 mb-8",
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
                }

                UsersList {}
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
