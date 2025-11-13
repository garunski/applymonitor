//! AI Metrics dashboard component

use dioxus::prelude::*;

#[component]
pub fn AiMetrics() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-2xl font-bold text-gray-900 dark:text-gray-100",
                "AI Processing Metrics"
            }
            div {
                class: "p-5 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                p {
                    class: "text-gray-600 dark:text-gray-400",
                    "Metrics dashboard coming soon. This will show processing statistics, category breakdown, and confidence metrics."
                }
            }
        }
    }
}
