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

    /// Register new user
    pub fn register(&self, email: String, password: String, name: String) {
        let mut user = self.user;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match AuthService::register(&email, &password, &name).await {
                Ok(registered_user) => {
                    *user.write() = Some(registered_user);
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

    /// Login with local username/password
    pub fn login_local(&self, email: String, password: String) {
        let mut user = self.user;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match AuthService::login_local(&email, &password).await {
                Ok(logged_in_user) => {
                    *user.write() = Some(logged_in_user);
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

    /// Link provider to account
    pub fn link_provider(&self, provider: String) {
        #[cfg(target_arch = "wasm32")]
        {
            let auth_url = AuthService::link_provider_url(&provider);
                        let window = web_sys::window().expect("no global `window` exists");
                        let location = window.location();
                        let _ = location.set_href(&auth_url);
                    }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = provider; // Suppress unused variable warning on non-WASM targets
        }
    }

    /// Unlink provider from account
    pub fn unlink_provider(&self, provider: String) {
        let mut user = self.user;
        let mut loading = self.loading;
        let mut error = self.error;

        spawn(async move {
            *loading.write() = true;
            *error.write() = None;

            match AuthService::unlink_provider(&provider).await {
                Ok(_) => {
                    // Refresh user to get updated providers list
                    match AuthService::fetch_current_user().await {
                        Ok(updated_user) => {
                            *user.write() = Some(updated_user);
                            *error.write() = None;
                        }
                        Err(e) => {
                            *error.write() = Some(e);
                        }
                    }
                }
                Err(e) => {
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
