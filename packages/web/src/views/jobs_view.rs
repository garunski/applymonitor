//! Web-specific jobs view

use dioxus::prelude::*;
use ui::{state::use_jobs_provider, JobsList};

/// Web-specific jobs view wrapper
#[component]
pub fn Jobs() -> Element {
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
