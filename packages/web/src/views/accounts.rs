use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_navigator;
use ui::{components::account_linking::AccountLinking, use_auth};

#[component]
pub fn Accounts() -> Element {
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
        document::Title { "Account Settings - ApplyMonitor" }
        document::Meta {
            name: "description",
            content: "Manage your account settings and connected providers.",
        }

        div {
            class: "container mx-auto px-4 sm:px-6 lg:px-8 py-6 max-w-4xl",
            h1 {
                class: "text-3xl font-bold text-gray-900 dark:text-white mb-6",
                "Account Settings"
            }
            AccountLinking {}
        }
    }
}
