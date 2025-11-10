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

                    let auth_url = AuthService::get_oauth_url("google");
                                let window = web_sys::window().expect("no global `window` exists");
                                let location = window.location();
                                let _ = location.set_href(&auth_url);
                }
            },
            "Sign in with Google"
        }
    }
}
