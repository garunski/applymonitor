//! Web-specific dashboard view

use crate::Route;
use dioxus::prelude::*;
use ui::{state::use_jobs_provider, DashboardContent};

/// Web-specific dashboard view wrapper
#[component]
pub fn Dashboard() -> Element {
    // Provide jobs state context at the top level
    use_jobs_provider();

    rsx! {
        document::Title { "Dashboard - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "View your job application statistics and recent applications. Track your progress and stay organized.",
        }
        document::Meta {
            property: "og:title",
            content: "Dashboard - ApplyMonitor",
        }
        document::Meta {
            property: "og:description",
            content: "View your job application statistics and recent applications. Track your progress and stay organized.",
        }

        DashboardContent {}

        // Quick action: View all jobs
        div {
            class: "px-4 sm:px-6 lg:px-8 pb-6",
            div {
                class: "flex justify-center",
                Link {
                    to: Route::Jobs {},
                    class: "text-sm font-medium text-brand-600 hover:text-brand-500 dark:text-brand-400 dark:hover:text-brand-300",
                    "View all jobs →"
                }
            }
        }
    }
}
