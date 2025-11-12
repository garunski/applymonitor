//! Web-specific emails view

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{state::use_emails_provider, state::use_jobs_provider, use_auth, EmailsList};

/// Web-specific emails view wrapper
#[component]
pub fn Emails() -> Element {
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

    // Provide emails and jobs state context at the top level
    use_emails_provider();
    use_jobs_provider();

    rsx! {
        document::Title { "Emails - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "View and manage emails scanned from your Gmail account. Create job applications from emails.",
        }
        document::Meta {
            property: "og:title",
            content: "Emails - ApplyMonitor",
        }
        document::Meta {
            property: "og:description",
            content: "View and manage emails scanned from your Gmail account. Create job applications from emails.",
        }

        EmailsList {}
    }
}
