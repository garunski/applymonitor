use crate::components::button::{Button, ButtonVariant};
use crate::state::auth_state::use_auth;
use dioxus::prelude::*;

/// Login page component following Catalyst Tailwind UI design
#[component]
pub fn LoginPage() -> Element {
    let auth = use_auth();
    let mut show_local = use_signal(|| false);
    let mut show_register = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut name = use_signal(String::new);

    rsx! {
        div {
            class: "flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8",
            div {
                class: "sm:mx-auto sm:w-full sm:max-w-md",
                div {
                    class: "flex justify-center",
                    h1 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white",
                        "ApplyMonitor"
                    }
                }
                h2 {
                    class: "mt-6 text-center text-3xl font-bold leading-9 tracking-tight text-gray-900 dark:text-white",
                    if *show_register.read() {
                        "Create your account"
                    } else {
                        "Sign in to your account"
                    }
                }
            }

            div {
                class: "mt-10 sm:mx-auto sm:w-full sm:max-w-[480px]",
                div {
                    class: "bg-white dark:bg-gray-800 px-6 py-12 shadow sm:rounded-lg sm:px-12 border border-gray-200 dark:border-gray-700",
                    if !*show_local.read() {
                        // OAuth providers
                        div {
                            class: "space-y-4",
                            // Google Provider
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "w-full flex items-center justify-center gap-3 text-base px-6 py-3 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700",
                                onclick: move |_| {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        use crate::services::auth_service::AuthService;

                                        let auth_url = AuthService::get_oauth_url("google");
                                                    let window = web_sys::window().expect("no global `window` exists");
                                                    let location = window.location();
                                                    let _ = location.set_href(&auth_url);
                                    }
                                },
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        fill: "currentColor",
                                        d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                                    }
                                    path {
                                        fill: "currentColor",
                                        d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                                    }
                                    path {
                                        fill: "currentColor",
                                        d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                                    }
                                    path {
                                        fill: "currentColor",
                                        d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                                    }
                                }
                                "Continue with Google"
                            }

                            // Divider
                            div {
                                class: "relative my-6",
                                div {
                                    class: "absolute inset-0 flex items-center",
                                    div {
                                        class: "w-full border-t border-gray-300 dark:border-gray-600"
                                    }
                                }
                                div {
                                    class: "relative flex justify-center text-sm",
                                    span {
                                        class: "bg-white dark:bg-gray-800 px-2 text-gray-500 dark:text-gray-400",
                                        "Or continue with"
                                    }
                                }
                            }

                            // Local login button
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "w-full flex items-center justify-center gap-3 text-base px-6 py-3 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700",
                                onclick: move |_| {
                                    *show_local.write() = true;
                                    *show_register.write() = false;
                                },
                                "Sign in with Email"
                            }
                        }
                    } else {
                        // Local login/register form
                        div {
                            class: "space-y-6",
                            if *show_register.read() {
                                // Registration form
                                div {
                                    class: "space-y-4",
                                    div {
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                            "Name"
                                        }
                                        input {
                                            class: "mt-1 block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500",
                                            r#type: "text",
                                            value: name,
                                            oninput: move |e| *name.write() = e.value(),
                                            placeholder: "Your name",
                                        }
                                    }
                                    div {
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                            "Email"
                                        }
                                        input {
                                            class: "mt-1 block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500",
                                            r#type: "email",
                                            value: email,
                                            oninput: move |e| *email.write() = e.value(),
                                            placeholder: "you@example.com",
                                        }
                                    }
                                    div {
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                            "Password"
                                        }
                                        input {
                                            class: "mt-1 block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500",
                                            r#type: "password",
                                            value: password,
                                            oninput: move |e| *password.write() = e.value(),
                                            placeholder: "••••••••",
                                        }
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        class: "w-full",
                                        onclick: move |_| {
                                            if !email().is_empty() && !password().is_empty() && !name().is_empty() {
                                                auth.register(email(), password(), name());
                                            }
                                        },
                                        "Create Account"
                                    }
                                    div {
                                        class: "text-center text-sm",
                                        "Already have an account? "
                                        button {
                                            class: "font-semibold text-blue-600 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300",
                                            onclick: move |_| {
                                                *show_register.write() = false;
                                            },
                                            "Sign in"
                                        }
                                    }
                                }
                            } else {
                                // Login form
                                div {
                                    class: "space-y-4",
                                    div {
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                            "Email"
                                        }
                                        input {
                                            class: "mt-1 block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500",
                                            r#type: "email",
                                            value: email,
                                            oninput: move |e| *email.write() = e.value(),
                                            placeholder: "you@example.com",
                                        }
                                    }
                                    div {
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                            "Password"
                                        }
                                        input {
                                            class: "mt-1 block w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500",
                                            r#type: "password",
                                            value: password,
                                            oninput: move |e| *password.write() = e.value(),
                                            placeholder: "••••••••",
                                        }
                                    }
                                    div {
                                        class: "flex items-center justify-between",
                                        div {
                                            class: "text-sm",
                                            a {
                                                href: "#",
                                                class: "font-semibold text-blue-600 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300",
                                                "Forgot password?"
                                            }
                                        }
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        class: "w-full",
                                        onclick: move |_| {
                                            if !email().is_empty() && !password().is_empty() {
                                                auth.login_local(email(), password());
                                            }
                                        },
                                        "Sign in"
                                    }
                                    div {
                                        class: "text-center text-sm",
                                        "Don't have an account? "
                                        button {
                                            class: "font-semibold text-blue-600 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300",
                                            onclick: move |_| {
                                                *show_register.write() = true;
                                            },
                                            "Sign up"
                                        }
                                    }
                                }
                            }
                            div {
                                class: "mt-4 text-center",
                                button {
                                    class: "text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300",
                                    onclick: move |_| {
                                        *show_local.write() = false;
                                        *show_register.write() = false;
                                    },
                                    "← Back to OAuth providers"
                                }
                            }
                        }
                    }
                }

                if !*show_local.read() {
                    p {
                        class: "mt-10 text-center text-sm/6 text-gray-500 dark:text-gray-400",
                        "Don't have an account? "
                        a {
                            href: "#",
                            class: "font-semibold text-blue-600 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300",
                            "Sign up with any provider above"
                        }
                    }
                }
            }
        }
    }
}
