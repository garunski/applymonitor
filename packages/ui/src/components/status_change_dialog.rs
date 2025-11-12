//! Status change confirmation dialog

use crate::components::button::{Button, ButtonVariant};
use crate::services::jobs_service::JobStatus;
use dioxus::prelude::*;

#[component]
pub fn StatusChangeDialog(
    current_status: JobStatus,
    new_status: JobStatus,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            div {
                class: "flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0",
                // Backdrop
                div {
                    class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity dark:bg-gray-900 dark:bg-opacity-75",
                    onclick: move |_| on_cancel.call(()),
                }
                // Dialog panel
                div {
                    class: "relative transform overflow-hidden rounded-lg bg-white dark:bg-gray-800 px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6",
                    div {
                        class: "sm:flex sm:items-start",
                        div {
                            class: "mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left",
                            h3 {
                                class: "text-base font-semibold leading-6 text-gray-900 dark:text-white",
                                "Change Status"
                            }
                            div {
                                class: "mt-2",
                                p {
                                    class: "text-sm text-gray-500 dark:text-gray-400",
                                    "Change status from "
                                    span {
                                        class: "font-medium text-gray-900 dark:text-white",
                                        {current_status.display_name.clone()}
                                    }
                                    " to "
                                    span {
                                        class: "font-medium text-gray-900 dark:text-white",
                                        {new_status.display_name.clone()}
                                    }
                                    "?"
                                }
                            }
                        }
                    }
                    div {
                        class: "mt-5 sm:mt-4 sm:flex sm:flex-row-reverse gap-3",
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| on_confirm.call(()),
                            "Confirm"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
