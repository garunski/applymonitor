//! Web-specific jobs view

use dioxus::prelude::*;
use ui::{state::use_jobs_provider, JobsList};

/// Web-specific jobs view wrapper
#[component]
pub fn Jobs() -> Element {
    // Provide jobs state context at the top level
    use_jobs_provider();

    rsx! {
        JobsList {}
    }
}
