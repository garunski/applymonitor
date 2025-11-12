//! Timeline component for displaying job events

use dioxus::prelude::*;
use serde_json::Value;

/// Timeline event types
#[derive(Debug, Clone, PartialEq)]
pub enum TimelineEventType {
    JobCreated,
    StatusChanged,
    EmailReceived,
    CommentAdded,
}

impl TimelineEventType {
    fn from_str(s: &str) -> Self {
        match s {
            "job_created" => TimelineEventType::JobCreated,
            "status_changed" => TimelineEventType::StatusChanged,
            "email_received" => TimelineEventType::EmailReceived,
            "comment_added" => TimelineEventType::CommentAdded,
            _ => TimelineEventType::JobCreated,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            TimelineEventType::JobCreated => "created the job",
            TimelineEventType::StatusChanged => "changed the status",
            TimelineEventType::EmailReceived => "received an email",
            TimelineEventType::CommentAdded => "commented",
        }
    }
}

/// Format a relative time string (e.g., "7d ago")
fn format_relative_time(timestamp: &str) -> String {
    // Try to parse the timestamp
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(dt.with_timezone(&chrono::Utc));

        if duration.num_days() > 0 {
            format!("{}d ago", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{}h ago", duration.num_hours())
        } else if duration.num_minutes() > 0 {
            format!("{}m ago", duration.num_minutes())
        } else {
            "just now".to_string()
        }
    } else {
        timestamp.to_string()
    }
}

/// Timeline component
#[component]
pub fn Timeline(events: Vec<Value>) -> Element {
    if events.is_empty() {
        return rsx! {
            div {
                class: "text-center py-8 text-gray-500 dark:text-gray-400",
                "No events yet"
            }
        };
    }

    rsx! {
        ul {
            role: "list",
            class: "space-y-6",
            for (idx, event) in events.iter().enumerate() {
                TimelineEvent {
                    event: event.clone(),
                    is_last: idx == events.len() - 1,
                }
            }
        }
    }
}

/// Individual timeline event component
#[component]
fn TimelineEvent(event: Value, is_last: bool) -> Element {
    let event_type_str = event
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("job_created");
    let event_type = TimelineEventType::from_str(event_type_str);
    let timestamp = event
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let data = event.get("data").cloned().unwrap_or(serde_json::json!({}));

    let relative_time = format_relative_time(timestamp);

    match event_type {
        TimelineEventType::CommentAdded => {
            let content = data.get("content").and_then(|v| v.as_str()).unwrap_or("");
            let user_name = data.get("user_name").and_then(|v| v.as_str());
            let user_picture = data.get("user_picture").and_then(|v| v.as_str());

            rsx! {
                li {
                    class: "relative flex gap-x-4",
                    if !is_last {
                        div {
                            class: "absolute -bottom-6 left-0 top-0 flex w-6 justify-center",
                            div {
                                class: "w-px bg-gray-200 dark:bg-white/15",
                            }
                        }
                    }
                    if let Some(picture) = user_picture {
                        img {
                            src: picture,
                            alt: "",
                            class: "relative mt-3 size-6 flex-none rounded-full bg-gray-50 outline outline-1 -outline-offset-1 outline-black/5 dark:bg-gray-800 dark:outline-white/10",
                        }
                    } else {
                        div {
                            class: "relative mt-3 size-6 flex-none rounded-full bg-gray-50 outline outline-1 -outline-offset-1 outline-black/5 dark:bg-gray-800 dark:outline-white/10",
                        }
                    }
                    div {
                        class: "flex-auto rounded-md p-3 ring-1 ring-inset ring-gray-200 dark:ring-white/15",
                        div {
                            class: "flex justify-between gap-x-4",
                            div {
                                class: "py-0.5 text-xs/5 text-gray-500 dark:text-gray-400",
                                if let Some(name) = user_name {
                                    span {
                                        class: "font-medium text-gray-900 dark:text-white",
                                        {name}
                                    }
                                    " commented"
                                } else {
                                    "Someone commented"
                                }
                            }
                            time {
                                datetime: timestamp,
                                class: "flex-none py-0.5 text-xs/5 text-gray-500 dark:text-gray-400",
                                {relative_time}
                            }
                        }
                        p {
                            class: "text-sm/6 text-gray-500 dark:text-gray-400",
                            {content}
                        }
                    }
                }
            }
        }
        _ => {
            let label = event_type.label();
            let display_text = match event_type {
                TimelineEventType::JobCreated => {
                    let title = data.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    format!("created the job: {}", title)
                }
                TimelineEventType::StatusChanged => {
                    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
                    format!("changed status to {}", status)
                }
                TimelineEventType::EmailReceived => {
                    let subject = data.get("subject").and_then(|v| v.as_str()).unwrap_or("");
                    format!("received email: {}", subject)
                }
                _ => label.to_string(),
            };

            rsx! {
                li {
                    class: "relative flex gap-x-4",
                    if !is_last {
                        div {
                            class: "absolute -bottom-6 left-0 top-0 flex w-6 justify-center",
                            div {
                                class: "w-px bg-gray-200 dark:bg-white/15",
                            }
                        }
                    } else {
                        div {
                            class: "absolute left-0 top-0 flex h-6 w-6 justify-center",
                            div {
                                class: "w-px bg-gray-200 dark:bg-white/15",
                            }
                        }
                    }
                    div {
                        class: "relative flex size-6 flex-none items-center justify-center bg-white dark:bg-gray-900",
                        if event_type == TimelineEventType::StatusChanged && is_last {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                "data-slot": "icon",
                                "aria-hidden": "true",
                                class: "size-6 text-indigo-600 dark:text-indigo-500",
                                path {
                                    d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z",
                                    "clip-rule": "evenodd",
                                    "fill-rule": "evenodd",
                                }
                            }
                        } else {
                            div {
                                class: "size-1.5 rounded-full bg-gray-100 ring ring-gray-300 dark:bg-white/10 dark:ring-white/20",
                            }
                        }
                    }
                    p {
                        class: "flex-auto py-0.5 text-xs/5 text-gray-500 dark:text-gray-400",
                        span {
                            class: "font-medium text-gray-900 dark:text-white",
                            "Saved"
                        }
                        " {display_text}"
                    }
                    time {
                        datetime: timestamp,
                        class: "flex-none py-0.5 text-xs/5 text-gray-500 dark:text-gray-400",
                        {relative_time}
                    }
                }
            }
        }
    }
}
