//! Auth state management

use crate::services::{
    auth_service::{AuthService, User},
    error::ServiceError,
};
use dioxus::prelude::*;

/// Auth state containing signals for user, loading, and error states
#[derive(Clone, Copy)]
pub struct AuthState {
    pub user: Signal<Option<User>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<ServiceError>>,
}

/// Provide auth state context to the component tree
pub fn use_auth_provider() -> AuthState {
    let user = use_signal(|| None::<User>);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<ServiceError>);

    let state = AuthState {
        user,
        loading,
        error,
    };
    use_context_provider(|| state);
    state
}

/// Consume auth state context from the component tree
pub fn use_auth() -> AuthState {
    use_context::<AuthState>()
}

impl AuthState {
    /// Fetch current user from /api/me
    pub fn fetch_user(&self) {
        let mut user = self.user;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match AuthService::fetch_current_user().await {
                Ok(fetched_user) => {
                    *user.write() = Some(fetched_user);
                    *error.write() = None;
                }
                Err(e) => {
                    *user.write() = None;
                    *error.write() = Some(e);
                }
            }

            *loading.write() = false;
        });
    }

    /// Logout user
    pub fn logout(&self) {
        let mut user = self.user;
        let mut error = self.error;

        // Clear user state
        *user.write() = None;
        *error.write() = None;

        // Redirect to logout URL
        #[cfg(target_arch = "wasm32")]
        {
            let logout_url = AuthService::logout_url();
            let window = web_sys::window().expect("no global `window` exists");
            let location = window.location();
            let _ = location.set_href(&logout_url);
        }
    }
}
