//! Web-specific jobs view

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{state::use_jobs, use_auth, JobsList};

/// Web-specific jobs view wrapper
#[component]
pub fn Jobs() -> Element {
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

    // Consume jobs state (provided at App level)
    let jobs_state = use_jobs();

    // Watch for created job ID and navigate
    use_effect({
        let mut created_job_id = jobs_state.created_job_id;
        move || {
            let job_id_opt = created_job_id.read().clone();
            if let Some(job_id) = job_id_opt {
                navigator.push(Route::JobDetails { id: job_id });
                // Reset the signal after navigation
                *created_job_id.write() = None;
            }
        }
    });

    rsx! {
        document::Title { "Jobs - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "View and manage all your job applications. Track application status, schedule follow-ups, and never lose track of an opportunity.",
        }
        document::Meta {
            property: "og:title",
            content: "Jobs - ApplyMonitor",
        }
        document::Meta {
            property: "og:description",
            content: "View and manage all your job applications. Track application status, schedule follow-ups, and never lose track of an opportunity.",
        }

        JobsList {}
    }
}
