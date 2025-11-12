//! Admin dashboard view

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{components::admin::AdminDashboard, use_auth};

/// Admin dashboard view with route protection
#[component]
pub fn Admin() -> Element {
    let auth = use_auth();
    let navigator = use_navigator();

    // Redirect to login if not authenticated
    use_effect(move || {
        let user = auth.user;
        let loading = auth.loading;

        if !loading() && user().is_none() {
            navigator.push(Route::Login {});
        }
    });

    // Check if user is admin
    let user = auth.user;
    let is_admin = user().as_ref().and_then(|u| u.is_admin).unwrap_or(false);

    // Redirect non-admin users
    use_effect(move || {
        let user = auth.user;
        let loading = auth.loading;

        if !loading() {
            if let Some(user_data) = user() {
                if !user_data.is_admin.unwrap_or(false) {
                    navigator.push(Route::Dashboard {});
                }
            }
        }
    });

    rsx! {
        document::Title { "Admin Dashboard - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "Admin dashboard for managing users and system settings.",
        }

        if !is_admin {
            div {
                class: "px-4 sm:px-6 lg:px-8 py-6",
                div {
                    class: "text-center",
                    p {
                        class: "text-gray-900 dark:text-gray-100",
                        "You do not have permission to access this page."
                    }
                }
            }
        } else {
            AdminDashboard {}
        }
    }
}
