use dioxus::prelude::*;
use ui::components::account_linking::AccountLinking;

#[component]
pub fn Accounts() -> Element {
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
