//! Auth API service

use crate::services::{api_config::get_api_base_url, error::ServiceError};
use serde::{Deserialize, Serialize};

/// User struct matching API response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i64,
    pub provider: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub created_at: Option<String>,
    pub last_login: Option<String>,
}

/// Auth API service
pub struct AuthService;

impl AuthService {
    /// Fetch current user from /api/me
    pub async fn fetch_current_user() -> Result<User, ServiceError> {
        let url = format!("{}/api/me", get_api_base_url());

        match gloo_net::http::Request::get(&url).send().await {
            Ok(response) => {
                if response.status() == 200 {
                    match response.json::<User>().await {
                        Ok(user) => Ok(user),
                        Err(e) => Err(ServiceError::Parse(format!("Failed to parse user: {}", e))),
                    }
                } else if response.status() == 401 {
                    Err(ServiceError::Unauthorized)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ServiceError::Server(status, text))
                }
            }
            Err(e) => Err(ServiceError::Network(format!(
                "Failed to fetch user: {}",
                e
            ))),
        }
    }

    /// Get OAuth URL for a specific provider by calling the API
    pub async fn get_oauth_url(provider: &str) -> Result<String, ServiceError> {
        let url = format!("{}/auth/login?provider={}", get_api_base_url(), provider);

        match gloo_net::http::Request::get(&url).send().await {
            Ok(response) => {
                if response.status() == 200 {
                    #[derive(serde::Deserialize)]
                    struct LoginResponse {
                        auth_url: String,
                    }
                    match response.json::<LoginResponse>().await {
                        Ok(data) => Ok(data.auth_url),
                        Err(e) => Err(ServiceError::Parse(format!(
                            "Failed to parse login response: {}",
                            e
                        ))),
                    }
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ServiceError::Server(status, text))
                }
            }
            Err(e) => Err(ServiceError::Network(format!(
                "Failed to fetch OAuth URL: {}",
                e
            ))),
        }
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
}
