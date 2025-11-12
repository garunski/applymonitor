//! Comment form component for adding and displaying comments

use crate::services::comments_service::Comment;
use crate::state::{use_auth, use_comments};
use crate::utils::format_relative_time;
use dioxus::prelude::*;

/// Comment form component
#[component]
pub fn CommentForm(job_id: String) -> Element {
    let mut comment_text = use_signal(String::new);
    let comments_state = use_comments();

    let handle_submit = {
        let job_id_clone = job_id.clone();
        let mut comment_text_signal = comment_text;
        let comments = comments_state;

        move |e: Event<FormData>| {
            e.prevent_default();
            let text = comment_text_signal().trim().to_string();

            if text.is_empty() {
                return;
            }

            let job_id_for_spawn = job_id_clone.clone();
            comments.add_comment(job_id_for_spawn, text.clone());
            *comment_text_signal.write() = String::new();
        }
    };

    let error_msg = comments_state
        .error
        .read()
        .as_ref()
        .map(|e| format!("Failed: {}", e));

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
                            disabled: *comments_state.loading.read() || comment_text().trim().is_empty(),
                            class: "rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-white/10 dark:text-white dark:shadow-none dark:ring-white/5 dark:hover:bg-white/20",
                            if *comments_state.loading.read() {
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
            if !comments_state.comments.read().is_empty() {
                ul {
                    role: "list",
                    class: "space-y-6",
                    for comment in comments_state.comments.read().iter() {
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
    let auth = use_auth();
    let user_name = comment.name.as_deref().unwrap_or("Unknown").to_string();
    let user_picture = comment.picture.clone();
    let content = comment.content.clone();
    let created_at = comment.created_at.clone();

    // Format relative time
    let user = auth.user.read();
    let timezone = user.as_ref().and_then(|u| u.timezone.as_deref());
    let relative_time = format_relative_time(&created_at, timezone);

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
