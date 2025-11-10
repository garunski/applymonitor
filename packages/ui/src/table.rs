//! Custom table component

use dioxus::prelude::*;

/// Table component for displaying tabular data
#[component]
pub fn Table(children: Element) -> Element {
    rsx! {
        div {
            class: "overflow-x-auto",
            table {
                class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                {children}
            }
        }
    }
}

/// Table header component
#[component]
pub fn TableHeader(children: Element) -> Element {
    rsx! {
        thead {
            class: "bg-gray-50 dark:bg-gray-800",
            {children}
        }
    }
}

/// Table body component
#[component]
pub fn TableBody(children: Element) -> Element {
    rsx! {
        tbody {
            class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
            {children}
        }
    }
}

/// Table row component
#[component]
pub fn TableRow(children: Element) -> Element {
    rsx! {
        tr {
            class: "hover:bg-gray-50 dark:hover:bg-gray-800",
            {children}
        }
    }
}

/// Table header cell component
#[component]
pub fn TableHeaderCell(children: Element) -> Element {
    rsx! {
        th {
            class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
            {children}
        }
    }
}

/// Table cell component
#[component]
pub fn TableCell(children: Element) -> Element {
    rsx! {
        td {
            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100",
            {children}
        }
    }
}
