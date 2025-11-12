//! Admin API service

use crate::services::{api_config::get_api_base_url, error::ServiceError, http_client};
use serde::{Deserialize, Serialize};

use super::auth_service::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminStats {
    pub total_users: i32,
    pub enabled_users: i32,
    pub disabled_users: i32,
    pub admin_count: i32,
}

/// Admin API service
pub struct AdminService;

impl AdminService {
    /// List all users (admin only)
    pub async fn list_users() -> Result<Vec<User>, ServiceError> {
        let url = format!("{}/api/admin/users", get_api_base_url());

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Vec<User>>(response).await
        } else if status == 401 || status == 403 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Update user enabled status (admin only)
    pub async fn update_user_enabled(user_id: &str, enabled: bool) -> Result<(), ServiceError> {
        let url = format!("{}/api/admin/users/{}/enabled", get_api_base_url(), user_id);

        #[derive(serde::Serialize)]
        struct UpdateRequest {
            enabled: bool,
        }

        let request_body = UpdateRequest { enabled };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::patch(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            Ok(())
        } else if status == 401 || status == 403 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Get admin statistics (admin only)
    pub async fn get_stats() -> Result<AdminStats, ServiceError> {
        let url = format!("{}/api/admin/stats", get_api_base_url());

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<AdminStats>(response).await
        } else if status == 401 || status == 403 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }
}
