//! Company and location editing components

use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::state::use_jobs;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsBuilding, BsGeoAlt, BsPencilSquare};
use dioxus_free_icons::Icon;

#[component]
pub fn CompanyField(
    job_id: String,
    company: String,
    editing: Signal<bool>,
    edit_value: Signal<String>,
) -> Element {
    let jobs_state = use_jobs();

    rsx! {
        if editing() {
            div {
                class: "flex items-center gap-2",
                label {
                    class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                    "Company"
                }
                Input {
                    id: "edit-company",
                    r#type: "text",
                    value: edit_value(),
                    oninput: move |e: Event<FormData>| {
                        let mut val = edit_value;
                        *val.write() = e.value();
                    },
                    class: "w-48",
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        let company_val = edit_value();
                        if !company_val.trim().is_empty() {
                            jobs_state.update_job_company(job_id.clone(), company_val);
                        }
                        let mut editing_signal = editing;
                        *editing_signal.write() = false;
                    },
                    "Save"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| {
                        let mut editing_signal = editing;
                        *editing_signal.write() = false;
                    },
                    "Cancel"
                }
            }
        } else {
            div {
                class: "flex items-center gap-2",
                Icon {
                    class: "h-4 w-4",
                    width: 16,
                    height: 16,
                    fill: "currentColor",
                    icon: BsBuilding,
                }
                span {
                    {company.clone()}
                }
                button {
                    class: "ml-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                    onclick: move |_| {
                        let mut val = edit_value;
                        *val.write() = company.clone();
                        let mut editing_signal = editing;
                        *editing_signal.write() = true;
                    },
                    Icon {
                        class: "h-3 w-3",
                        width: 12,
                        height: 12,
                        fill: "currentColor",
                        icon: BsPencilSquare,
                    }
                }
            }
        }
    }
}

#[component]
pub fn LocationField(
    job_id: String,
    location: Option<String>,
    editing: Signal<bool>,
    edit_value: Signal<String>,
) -> Element {
    let jobs_state = use_jobs();
    let location_clone = location.clone();

    rsx! {
        if let Some(ref loc) = location_clone {
            if editing() {
                div {
                    class: "flex items-center gap-2",
                    Input {
                        id: "edit-location",
                        r#type: "text",
                        value: edit_value(),
                        oninput: move |e: Event<FormData>| {
                            let mut val = edit_value;
                            *val.write() = e.value();
                        },
                        class: "w-48",
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            let location_val = edit_value();
                            let location_opt = if location_val.trim().is_empty() {
                                None
                            } else {
                                Some(location_val.trim().to_string())
                            };
                            jobs_state.update_job_location(job_id.clone(), location_opt);
                            let mut editing_signal = editing;
                            *editing_signal.write() = false;
                        },
                        "Save"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            let mut editing_signal = editing;
                            *editing_signal.write() = false;
                        },
                        "Cancel"
                    }
                }
            } else {
                div {
                    class: "flex items-center gap-2",
                    Icon {
                        class: "h-4 w-4",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsGeoAlt,
                    }
                    span {
                        {loc.clone()}
                    }
                    button {
                        class: "ml-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                        onclick: {
                            let loc_value = loc.clone();
                            move |_| {
                                let mut val = edit_value;
                                *val.write() = loc_value.clone();
                                let mut editing_signal = editing;
                                *editing_signal.write() = true;
                            }
                        },
                        Icon {
                            class: "h-3 w-3",
                            width: 12,
                            height: 12,
                            fill: "currentColor",
                            icon: BsPencilSquare,
                        }
                    }
                }
            }
        } else if editing() {
            div {
                class: "flex items-center gap-2",
                Input {
                    id: "edit-location",
                    r#type: "text",
                    value: edit_value(),
                    oninput: move |e: Event<FormData>| {
                        let mut val = edit_value;
                        *val.write() = e.value();
                    },
                    class: "w-48",
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        let location_val = edit_value();
                        let location_opt = if location_val.trim().is_empty() {
                            None
                        } else {
                            Some(location_val.trim().to_string())
                        };
                        jobs_state.update_job_location(job_id.clone(), location_opt);
                        let mut editing_signal = editing;
                        *editing_signal.write() = false;
                    },
                    "Save"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| {
                        let mut editing_signal = editing;
                        *editing_signal.write() = false;
                    },
                    "Cancel"
                }
            }
        } else {
            div {
                class: "flex items-center gap-2",
                Icon {
                    class: "h-4 w-4",
                    width: 16,
                    height: 16,
                    fill: "currentColor",
                    icon: BsGeoAlt,
                }
                button {
                    class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                    onclick: move |_| {
                        let mut val = edit_value;
                        *val.write() = String::new();
                        let mut editing_signal = editing;
                        *editing_signal.write() = true;
                    },
                    "Add location"
                }
            }
        }
    }
}
