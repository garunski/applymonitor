//! Emails API service

use crate::services::{error::ServiceError, http_client};
use serde::{Deserialize, Serialize};

/// Stored email structure matching database schema
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct StoredEmail {
    pub gmail_id: String,
    pub user_id: String,
    pub scan_id: Option<String>,
    pub thread_id: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub snippet: Option<String>,
    pub date: Option<String>,
    pub created_at: Option<String>,
}

/// Request for assigning email to job
#[derive(Debug, Serialize)]
pub struct AssignJobRequest {
    pub job_id: Option<String>,
    pub create_job: Option<CreateJobFromEmail>,
}

/// Request for creating job from email
#[derive(Debug, Serialize)]
pub struct CreateJobFromEmail {
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: Option<String>,
}

/// Response from assign job endpoint
#[derive(Debug, Deserialize)]
pub struct AssignJobResponse {
    pub success: bool,
    pub gmail_id: String,
    pub job_id: String,
}

/// Emails API service
pub struct EmailsService;

impl EmailsService {
    /// Get the base URL for the Gmail scanner worker
    fn get_scanner_base_url() -> String {
        if let Ok(url) = std::env::var("GMAIL_SCANNER_URL") {
            if !url.is_empty() {
                return url;
            }
        }

        #[cfg(feature = "api-prod")]
        {
            return "https://applymonitor-gmail-scanner.workers.dev".to_string();
        }

        #[cfg(feature = "api-staging")]
        {
            return "https://applymonitor-gmail-scanner-staging.workers.dev".to_string();
        }

        "http://localhost:8001".to_string()
    }

    /// Fetch all emails for the current user
    pub async fn list_emails(
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<StoredEmail>, ServiceError> {
        let mut url = format!("{}/emails", Self::get_scanner_base_url());

        let mut query_params = Vec::new();
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(offset) = offset {
            query_params.push(format!("offset={}", offset));
        }

        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Vec<StoredEmail>>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Fetch a single email by ID
    pub async fn get_email(id: String) -> Result<StoredEmail, ServiceError> {
        let url = format!("{}/emails/{}", Self::get_scanner_base_url(), id);

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<StoredEmail>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Assign email to a job (create new or link to existing)
    pub async fn assign_email_to_job(
        gmail_id: String,
        request: AssignJobRequest,
    ) -> Result<AssignJobResponse, ServiceError> {
        let url = format!(
            "{}/emails/{}/assign-job",
            Self::get_scanner_base_url(),
            gmail_id
        );

        let body = serde_json::to_string(&request)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<AssignJobResponse>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }
}
