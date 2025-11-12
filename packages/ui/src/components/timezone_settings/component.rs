//! Timezone settings component

use crate::components::label::Label;
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectList, SelectOption, SelectTrigger, SelectValue,
};
use crate::state::use_auth;
use dioxus::prelude::*;

/// Common timezones grouped by region
const TIMEZONES: &[(&str, &[&str])] = &[
    (
        "Americas",
        &[
            "America/New_York",
            "America/Chicago",
            "America/Denver",
            "America/Los_Angeles",
            "America/Phoenix",
            "America/Anchorage",
            "America/Toronto",
            "America/Vancouver",
            "America/Mexico_City",
            "America/Sao_Paulo",
            "America/Buenos_Aires",
        ],
    ),
    (
        "Europe",
        &[
            "Europe/London",
            "Europe/Paris",
            "Europe/Berlin",
            "Europe/Rome",
            "Europe/Madrid",
            "Europe/Amsterdam",
            "Europe/Stockholm",
            "Europe/Warsaw",
            "Europe/Moscow",
        ],
    ),
    (
        "Asia",
        &[
            "Asia/Tokyo",
            "Asia/Shanghai",
            "Asia/Hong_Kong",
            "Asia/Singapore",
            "Asia/Dubai",
            "Asia/Kolkata",
            "Asia/Bangkok",
            "Asia/Seoul",
        ],
    ),
    (
        "Pacific",
        &["Pacific/Auckland", "Pacific/Sydney", "Pacific/Honolulu"],
    ),
];

/// Get browser timezone (web only)
fn get_browser_timezone() -> Option<String> {
    None
}

/// Timezone settings component
#[component]
pub fn TimezoneSettings() -> Element {
    let auth = use_auth();
    let user = auth.user.read().clone();
    let current_timezone = user.as_ref().and_then(|u| u.timezone.clone());
    // Use empty string for UTC/None, actual timezone string otherwise
    let mut selected_timezone = use_signal(|| current_timezone.clone().unwrap_or_default());

    // Update selected timezone when user changes
    use_effect(move || {
        let user = auth.user.read().clone();
        if let Some(user) = user {
            *selected_timezone.write() = user.timezone.clone().unwrap_or_default();
        }
    });

    // Get browser timezone as default suggestion
    let browser_tz = get_browser_timezone();

    // Create index signals for all options upfront
    let utc_index = use_signal(|| 0usize);
    let mut all_indices: Vec<Vec<Signal<usize>>> = Vec::new();
    for (region_idx, (_, tzs)) in TIMEZONES.iter().enumerate() {
        let region_start = (region_idx + 1) * 100;
        let mut region_indices = Vec::new();
        for (tz_idx, _) in tzs.iter().enumerate() {
            region_indices.push(use_signal(|| region_start + tz_idx));
        }
        all_indices.push(region_indices);
    }

    // Select expects ReadSignal<Option<Option<T>>>, so wrap in Option
    let mut select_value = use_signal(|| Some(Some(selected_timezone.read().clone())));
    use_effect(move || {
        *select_value.write() = Some(Some(selected_timezone.read().clone()));
    });

    let handle_change = {
        move |tz: Option<String>| {
            let tz_option = tz
                .as_ref()
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });
            auth.update_timezone(tz_option.clone());
            *selected_timezone.write() = tz.unwrap_or_default();
        }
    };

    rsx! {
        div {
            class: "px-4 sm:px-6 lg:px-8 py-6",
            div {
                class: "mb-6",
                h2 {
                    class: "text-base font-semibold text-gray-900 dark:text-white",
                    "Timezone"
                }
                p {
                    class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                    "Set your timezone to display dates and times in your local time."
                }
            }

            div {
                class: "space-y-4",
                Label {
                    html_for: "timezone",
                    "Timezone"
                }
                Select {
                    value: select_value,
                    on_value_change: handle_change,
                    placeholder: "Select timezone",
                    SelectTrigger {
                        id: "timezone",
                        SelectValue {}
                    }
                    SelectList {
                        SelectOption::<String> {
                            value: "".to_string(),
                            text_value: "UTC (default)".to_string(),
                            index: utc_index,
                            "UTC (default)"
                        }
                        for (region_idx, (region, tzs)) in TIMEZONES.iter().enumerate() {
                            SelectGroup {
                                SelectGroupLabel {
                                    {*region}
                                }
                                for (tz_idx, tz) in tzs.iter().enumerate() {
                                    SelectOption::<String> {
                                        value: tz.to_string(),
                                        text_value: tz.to_string(),
                                        index: all_indices[region_idx][tz_idx],
                                        {*tz}
                                    }
                                }
                            }
                        }
                    }
                }
                if !selected_timezone.read().is_empty() {
                    p {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "Selected: {selected_timezone.read()}"
                    }
                } else if let Some(ref browser_tz) = browser_tz {
                    p {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "Browser detected: {browser_tz} (not set)"
                    }
                }
            }
        }
    }
}
