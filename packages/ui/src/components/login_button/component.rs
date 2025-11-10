use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn LoginButton() -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Primary,
            class: "flex items-center gap-2",
            onclick: move |_| {
                #[cfg(target_arch = "wasm32")]
                {
                    use crate::services::auth_service::AuthService;
                    use dioxus::prelude::*;

                    spawn(async move {
                        match AuthService::get_oauth_url("google").await {
                            Ok(auth_url) => {
                                let window = web_sys::window().expect("no global `window` exists");
                                let location = window.location();
                                let _ = location.set_href(&auth_url);
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Failed to get OAuth URL: {:?}", e).into());
                            }
                        }
                    });
                }
            },
            "Sign in with Google"
        }
    }
}
