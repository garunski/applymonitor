//! Users list component for admin dashboard

use crate::services::{admin_service::AdminService, auth_service::User};
use dioxus::prelude::*;

#[component]
pub fn UsersList() -> Element {
    let mut users = use_signal(Vec::<User>::new);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);

    use_effect(move || {
        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match AdminService::list_users().await {
                Ok(user_list) => {
                    *users.write() = user_list;
                    *error.write() = None;
                }
                Err(e) => {
                    *error.write() = Some(format!("Failed to load users: {:?}", e));
                }
            }

            *loading.write() = false;
        });
    });

    rsx! {
        div {
            class: "px-4 sm:px-6 lg:px-8 py-6",
            h2 {
                class: "text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6",
                "Users"
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
                    "Loading users..."
                }
            } else {
                div {
                    class: "overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg",
                    table {
                        class: "min-w-full divide-y divide-gray-300 dark:divide-gray-700",
                        thead {
                            class: "bg-gray-50 dark:bg-gray-800",
                            tr {
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "Email"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "Name"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "Created"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "Status"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "Admin"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "Actions"
                                }
                            }
                        }
                        tbody {
                            class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                            for user in users().iter() {
                                UserRow {
                                    user: user.clone(),
                                    on_update: move |_| {
                                        let mut users_signal = users;
                                        spawn(async move {
                                            if let Ok(user_list) = AdminService::list_users().await {
                                                *users_signal.write() = user_list;
                                            }
                                        });
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

#[component]
fn UserRow(user: User, on_update: EventHandler<()>) -> Element {
    let updating = use_signal(|| false);
    let enabled = user.enabled.unwrap_or(true);
    let user_id = user.id.clone();
    let enabled_value = enabled;

    let toggle_enabled = move |_| {
        let user_id = user_id.clone();
        let mut updating_signal = updating;
        let on_update_handler = on_update;
        spawn(async move {
            *updating_signal.write() = true;

            match AdminService::update_user_enabled(&user_id, !enabled_value).await {
                Ok(_) => {
                    on_update_handler.call(());
                }
                Err(_e) => {
                    // Error updating user - will be handled by UI state
                }
            }

            *updating_signal.write() = false;
        });
    };

    rsx! {
        tr {
            td {
                class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100",
                {user.email.as_deref().unwrap_or("N/A")}
            }
            td {
                class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100",
                {user.name.as_deref().unwrap_or("N/A")}
            }
            td {
                class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                if let Some(created) = user.created_at.as_deref() {
                    {created}
                } else {
                    "N/A"
                }
            }
            td {
                class: "px-6 py-4 whitespace-nowrap",
                if enabled {
                    span {
                        class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
                        "Enabled"
                    }
                } else {
                    span {
                        class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
                        "Disabled"
                    }
                }
            }
            td {
                class: "px-6 py-4 whitespace-nowrap",
                if user.is_admin.unwrap_or(false) {
                    span {
                        class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
                        "Admin"
                    }
                } else {
                    span {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "—"
                    }
                }
            }
            td {
                class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                button {
                    class: if enabled {
                        "text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
                    } else {
                        "text-green-600 hover:text-green-900 dark:text-green-400 dark:hover:text-green-300"
                    },
                    disabled: updating(),
                    onclick: toggle_enabled,
                    if updating() {
                        "Updating..."
                    } else if enabled {
                        "Disable"
                    } else {
                        "Enable"
                    }
                }
            }
        }
    }
}
