//! Comment form component for adding and displaying comments

use crate::services::comments_service::{Comment, CommentsService};
use dioxus::prelude::*;

/// Comment form component
#[component]
pub fn CommentForm(job_id: String) -> Element {
    let mut comment_text = use_signal(String::new);
    let comments = use_signal(Vec::<Comment>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<String>);

    // Fetch comments on mount
    use_effect({
        let job_id_clone = job_id.clone();
        let mut comments_signal = comments;
        let mut loading_signal = loading;
        let mut error_signal = error;

        move || {
            let job_id_for_fetch = job_id_clone.clone();
            spawn(async move {
                *loading_signal.write() = true;
                *error_signal.write() = None;

                match CommentsService::fetch_comments(job_id_for_fetch).await {
                    Ok(fetched_comments) => {
                        *comments_signal.write() = fetched_comments;
                        *error_signal.write() = None;
                    }
                    Err(e) => {
                        *error_signal.write() = Some(format!("Failed to load comments: {}", e));
                    }
                }

                *loading_signal.write() = false;
            });
        }
    });

    let handle_submit = {
        let job_id_clone = job_id.clone();
        let mut comment_text_signal = comment_text;
        let mut comments_signal = comments;
        let mut loading_signal = loading;
        let mut error_signal = error;

        move |e: Event<FormData>| {
            e.prevent_default();
            let text = comment_text_signal().trim().to_string();

            if text.is_empty() {
                return;
            }

            let job_id_for_spawn = job_id_clone.clone();
            spawn(async move {
                *loading_signal.write() = true;
                *error_signal.write() = None;

                match CommentsService::create_comment(job_id_for_spawn, text.clone()).await {
                    Ok(new_comment) => {
                        let mut comments_list = comments_signal.read().clone();
                        comments_list.insert(0, new_comment);
                        *comments_signal.write() = comments_list;
                        *comment_text_signal.write() = String::new();
                        *error_signal.write() = None;
                    }
                    Err(e) => {
                        *error_signal.write() = Some(format!("Failed to create comment: {}", e));
                    }
                }

                *loading_signal.write() = false;
            });
        }
    };

    let error_msg = error.read().as_ref().map(|e| e.to_string());

    rsx! {
        div {
            class: "space-y-6",
            // Comment form
            div {
                class: "flex gap-x-3",
                div {
                    class: "size-6 flex-none rounded-full bg-gray-50 outline outline-1 -outline-offset-1 outline-black/5 dark:bg-gray-800 dark:outline-white/10",
                }
                form {
                    action: "#",
                    class: "relative flex-auto",
                    onsubmit: handle_submit,
                    div {
                        class: "overflow-hidden rounded-lg pb-12 outline outline-1 -outline-offset-1 outline-gray-300 focus-within:outline focus-within:outline-2 focus-within:-outline-offset-2 focus-within:outline-indigo-600 dark:bg-white/5 dark:outline-white/10 dark:focus-within:outline-indigo-500",
                        label {
                            "for": "comment",
                            class: "sr-only",
                            "Add your comment"
                        }
                        textarea {
                            id: "comment",
                            name: "comment",
                            rows: "2",
                            placeholder: "Add your comment...",
                            class: "block w-full resize-none bg-transparent px-3 py-1.5 text-base text-gray-900 placeholder:text-gray-400 focus:outline focus:outline-0 sm:text-sm/6 dark:text-white dark:placeholder:text-gray-500",
                            value: "{comment_text}",
                            oninput: move |e: Event<FormData>| {
                                *comment_text.write() = e.value();
                            },
                        }
                    }
                    div {
                        class: "absolute inset-x-0 bottom-0 flex justify-end py-2 pl-3 pr-2",
                        button {
                            r#type: "submit",
                            disabled: loading() || comment_text().trim().is_empty(),
                            class: "rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-white/10 dark:text-white dark:shadow-none dark:ring-white/5 dark:hover:bg-white/20",
                            if loading() {
                                "Posting..."
                            } else {
                                "Comment"
                            }
                        }
                    }
                }
            }

            // Error message
            if let Some(ref err_msg) = error_msg {
                div {
                    class: "rounded-md bg-red-50 dark:bg-red-900/20 p-4",
                    p {
                        class: "text-sm text-red-800 dark:text-red-200",
                        {err_msg.clone()}
                    }
                }
            }

            // Comments list
            if !comments.read().is_empty() {
                ul {
                    role: "list",
                    class: "space-y-6",
                    for comment in comments.read().iter() {
                        CommentItem {
                            comment: comment.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Individual comment item component
#[component]
fn CommentItem(comment: Comment) -> Element {
    let user_name = comment.name.as_deref().unwrap_or("Unknown").to_string();
    let user_picture = comment.picture.clone();
    let content = comment.content.clone();
    let created_at = comment.created_at.clone();

    // Format relative time
    let relative_time = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&created_at) {
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
        created_at.clone()
    };

    rsx! {
        li {
            class: "relative flex gap-x-4",
            if let Some(ref picture) = user_picture {
                img {
                    src: picture.clone(),
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
                        span {
                            class: "font-medium text-gray-900 dark:text-white",
                            {user_name}
                        }
                        " commented"
                    }
                    time {
                        datetime: created_at,
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
