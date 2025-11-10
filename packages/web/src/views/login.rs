use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{use_auth, LoginPage};

#[component]
pub fn Login() -> Element {
    let auth = use_auth();
    let navigator = use_navigator();

    // Check if user is already logged in and redirect to jobs
    // Note: User is already fetched by App component on mount, so we don't need to fetch again
    // This effect runs whenever the user state changes
    use_effect(move || {
        let user = auth.user;
        let loading = auth.loading;

        // Only redirect if we're not loading and user is logged in
        if !loading() && user().is_some() {
            navigator.push(Route::Jobs {});
        }
    });

    rsx! {
        document::Title { "Sign in - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "Sign in to your ApplyMonitor account to track and manage your job applications.",
        }

        LoginPage {}
    }
}
