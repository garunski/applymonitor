//! Web-specific jobs view

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{state::use_jobs_provider, use_auth, JobsList};

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

    // Provide jobs state context at the top level
    use_jobs_provider();

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
