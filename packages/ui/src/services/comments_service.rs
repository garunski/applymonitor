//! Comments API service

use crate::services::{api_config::get_api_base_url, error::ServiceError, http_client};
use serde::{Deserialize, Serialize};

/// Comment struct matching API response
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Comment {
    pub id: String,
    pub job_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub picture: Option<String>,
}

/// Request struct for creating a comment
#[derive(Debug, Serialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

/// Comments API service
pub struct CommentsService;

impl CommentsService {
    /// Fetch comments for a job
    pub async fn fetch_comments(job_id: String) -> Result<Vec<Comment>, ServiceError> {
        let url = format!("{}/jobs/{}/comments", get_api_base_url(), job_id);

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Vec<Comment>>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Create a new comment
    pub async fn create_comment(job_id: String, content: String) -> Result<Comment, ServiceError> {
        let url = format!("{}/jobs/{}/comments", get_api_base_url(), job_id);

        let request = CreateCommentRequest { content };
        let body = serde_json::to_string(&request)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize comment: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 201 || status == 200 {
            http_client::json::<Comment>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }
}
