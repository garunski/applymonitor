//! Badge component for status indicators

use dioxus::prelude::*;

/// Badge component for displaying status indicators
#[component]
pub fn Badge(status: String) -> Element {
    let (bg_color, text_color) = match status.as_str() {
        "applied" => (
            "bg-blue-100 dark:bg-blue-900",
            "text-blue-800 dark:text-blue-200",
        ),
        "interviewing" => (
            "bg-yellow-100 dark:bg-yellow-900",
            "text-yellow-800 dark:text-yellow-200",
        ),
        "rejected" => (
            "bg-red-100 dark:bg-red-900",
            "text-red-800 dark:text-red-200",
        ),
        "offer" => (
            "bg-green-100 dark:bg-green-900",
            "text-green-800 dark:text-green-200",
        ),
        "open" => (
            "bg-gray-100 dark:bg-gray-800",
            "text-gray-800 dark:text-gray-200",
        ),
        _ => (
            "bg-gray-100 dark:bg-gray-800",
            "text-gray-800 dark:text-gray-200",
        ),
    };

    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {bg_color} {text_color}",
            {status}
        }
    }
}
