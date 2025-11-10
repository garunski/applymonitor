use crate::state::use_auth;
use dioxus::prelude::*;

#[component]
pub fn UserProfile() -> Element {
    let auth = use_auth();
    let user = auth.user;

    if let Some(user) = user() {
        let initials = user
            .name
            .as_ref()
            .and_then(|n| {
                let chars: String = n
                    .split_whitespace()
                    .take(2)
                    .map(|s| s.chars().next().unwrap_or(' '))
                    .collect();
                if chars.len() >= 2 {
                    Some(chars.chars().take(2).collect())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                user.email
                    .as_ref()
                    .and_then(|e| e.chars().next().map(|c| c.to_uppercase().to_string()))
                    .unwrap_or_else(|| "U".to_string())
            });

        rsx! {
            div {
                class: "relative",
                button {
                    class: "flex items-center gap-2 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800",
                    onclick: move |_| {
                        // Toggle dropdown - simplified for now
                        // In a full implementation, you'd use a signal to track dropdown state
                    },
                    if let Some(ref avatar) = user.avatar {
                        img {
                            src: avatar.clone(),
                            alt: user.name.as_deref().unwrap_or("User"),
                            class: "h-8 w-8 rounded-full"
                        }
                    } else {
                        div {
                            class: "h-8 w-8 rounded-full bg-blue-600 dark:bg-blue-500 flex items-center justify-center text-white text-sm font-medium",
                            {initials}
                        }
                    }
                    span {
                        class: "hidden sm:block text-sm font-medium text-gray-700 dark:text-gray-300",
                        {user.name.as_deref().unwrap_or_else(|| user.email.as_deref().unwrap_or("User"))}
                    }
                    svg {
                        class: "h-4 w-4 text-gray-500 dark:text-gray-400",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path {
                            fill_rule: "evenodd",
                            d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z",
                            clip_rule: "evenodd"
                        }
                    }
                }

                // Dropdown menu (simplified - in production, use a proper dropdown component)
                div {
                    class: "absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 focus:outline-none z-10",
                    div {
                        class: "py-1",
                        button {
                            class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
                            onclick: move |_| {
                                auth.logout();
                            },
                            "Sign out"
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}
