//! Web-specific job details view

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{use_auth, JobDetails as JobDetailsComponent};

/// Web-specific job details view wrapper
#[component]
pub fn JobDetails(id: String) -> Element {
    let auth = use_auth();
    let navigator = use_navigator();

    // Fetch user on mount
    use_effect(move || {
        auth.fetch_user();
    });

    // Redirect to login if not authenticated
    use_effect(move || {
        let user = auth.user;
        let loading = auth.loading;

        if !loading() && user().is_none() {
            navigator.push(Route::Login {});
        }
    });

    rsx! {
        document::Title { "Job Details - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "View job application details, timeline, and comments.",
        }
        document::Meta {
            property: "og:title",
            content: "Job Details - ApplyMonitor",
        }
        document::Meta {
            property: "og:description",
            content: "View job application details, timeline, and comments.",
        }

        JobDetailsComponent {
            job_id: id,
        }
    }
}
