//! Auth API service

use crate::services::{api_config::get_api_base_url, error::ServiceError, http_client};
use serde::{Deserialize, Serialize};

/// User struct matching API response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: String, // UUID
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub providers: Vec<String>,
}

/// Auth API service
pub struct AuthService;

impl AuthService {
    /// Fetch current user from /api/me
    pub async fn fetch_current_user() -> Result<User, ServiceError> {
        let url = format!("{}/api/me", get_api_base_url());

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<User>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Get OAuth login URL - redirects directly to API which will redirect to OAuth provider
    /// This ensures cookies are set via navigation request, not fetch request
    pub fn get_oauth_url(provider: &str) -> String {
        format!("{}/auth/login?provider={}", get_api_base_url(), provider)
    }

    /// Get login URL for a specific provider (deprecated - use get_oauth_url instead)
    #[deprecated(note = "Use get_oauth_url instead")]
    pub fn login_url(provider: &str) -> String {
        format!("{}/auth/login?provider={}", get_api_base_url(), provider)
    }

    /// Get logout URL
    pub fn logout_url() -> String {
        format!("{}/auth/logout", get_api_base_url())
    }

    /// Register new user with local authentication
    pub async fn register(email: &str, password: &str, name: &str) -> Result<User, ServiceError> {
        let url = format!("{}/auth/register", get_api_base_url());

        #[derive(serde::Serialize)]
        struct RegisterRequest {
            email: String,
            password: String,
            name: String,
        }

        let request_body = RegisterRequest {
            email: email.to_string(),
            password: password.to_string(),
            name: name.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<User>(response).await
        } else if status == 409 {
            Err(ServiceError::Server(
                409,
                "Email already registered".to_string(),
            ))
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Login with local username/password
    pub async fn login_local(email: &str, password: &str) -> Result<User, ServiceError> {
        let url = format!("{}/auth/login/local", get_api_base_url());

        #[derive(serde::Serialize)]
        struct LoginRequest {
            email: String,
            password: String,
        }

        let request_body = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<User>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Request password reset
    pub async fn request_password_reset(email: &str) -> Result<(), ServiceError> {
        let url = format!("{}/auth/password-reset/request", get_api_base_url());

        #[derive(serde::Serialize)]
        struct ResetRequest {
            email: String,
        }

        let request_body = ResetRequest {
            email: email.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            Ok(())
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Confirm password reset
    pub async fn confirm_password_reset(
        token: &str,
        new_password: &str,
    ) -> Result<(), ServiceError> {
        let url = format!("{}/auth/password-reset/confirm", get_api_base_url());

        #[derive(serde::Serialize)]
        struct ConfirmResetRequest {
            token: String,
            new_password: String,
        }

        let request_body = ConfirmResetRequest {
            token: token.to_string(),
            new_password: new_password.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            Ok(())
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Link provider to existing account
    /// Get account linking URL - redirects directly to API which will redirect to OAuth provider
    /// This ensures cookies are set via navigation request, not fetch request
    pub fn link_provider_url(provider: &str) -> String {
        format!("{}/auth/link?provider={}", get_api_base_url(), provider)
    }

    /// Unlink provider from account
    pub async fn unlink_provider(provider: &str) -> Result<(), ServiceError> {
        let url = format!("{}/auth/unlink?provider={}", get_api_base_url(), provider);

        let response = http_client::post(&url, None).await?;
        let status = response.status();

        if status == 200 {
            Ok(())
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }
}
