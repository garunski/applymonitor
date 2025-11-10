use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::state::use_auth;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsChevronDown, Icon};

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
            DropdownMenu {
                DropdownMenuTrigger {
                    button {
                        class: "flex items-center gap-2 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800",
                        if let Some(ref picture) = user.picture {
                            img {
                                src: picture.clone(),
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
                        Icon {
                            class: "h-4 w-4 text-gray-500 dark:text-gray-400",
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: BsChevronDown,
                        }
                    }
                }
                DropdownMenuContent {
                    DropdownMenuItem::<String> {
                        index: use_signal(|| 0usize),
                        value: "profile".to_string(),
                        "Profile"
                    }
                    DropdownMenuItem::<String> {
                        index: use_signal(|| 1usize),
                        value: "settings".to_string(),
                        "Settings"
                    }
                    DropdownMenuItem::<String> {
                        index: use_signal(|| 2usize),
                        value: "signout".to_string(),
                        on_select: move |_| {
                            auth.logout();
                        },
                        "Sign out"
                    }
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}
