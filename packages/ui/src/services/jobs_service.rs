//! Jobs API service

use crate::services::{api_config::get_api_base_url, error::ServiceError, http_client};
use serde::{Deserialize, Serialize};

/// Job struct matching API response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Job {
    pub id: Option<String>,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Request struct for creating a job
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateJobRequest {
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
}

/// Request struct for updating a job
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateJobRequest {
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
}

/// Jobs API service
pub struct JobsService;

impl JobsService {
    /// Fetch all jobs
    pub async fn fetch_jobs() -> Result<Vec<Job>, ServiceError> {
        let url = format!("{}/jobs", get_api_base_url());

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Vec<Job>>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Fetch a single job by ID
    pub async fn fetch_job(id: String) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs/{}", get_api_base_url(), id);

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Job>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Create a new job
    pub async fn create_job(job: CreateJobRequest) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs", get_api_base_url());

        let body = serde_json::to_string(&job)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize job: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 201 || status == 200 {
            http_client::json::<Job>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Update an existing job
    pub async fn update_job(id: String, job: UpdateJobRequest) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs/{}", get_api_base_url(), id);

        let body = serde_json::to_string(&job)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize job: {}", e)))?;

        let response = http_client::put(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Job>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Delete a job
    pub async fn delete_job(id: String) -> Result<(), ServiceError> {
        let url = format!("{}/jobs/{}", get_api_base_url(), id);

        let response = http_client::delete(&url).await?;
        let status = response.status();

        if status == 200 || status == 204 {
            Ok(())
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Fetch job details with related data (emails, comments, timeline)
    pub async fn fetch_job_details(id: String) -> Result<JobDetails, ServiceError> {
        let url = format!("{}/jobs/{}?include=details", get_api_base_url(), id);

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<JobDetails>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Update job description only
    pub async fn update_job_description(
        id: String,
        description: Option<String>,
    ) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs/{}?description_only=true", get_api_base_url(), id);

        let body = serde_json::json!({ "description": description });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize: {}", e)))?;

        let response = http_client::put(&url, Some(&body_str)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Job>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }
}

/// Email contact information
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EmailContact {
    pub email: String,
    pub user_id: String,
    pub name: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
    pub is_system: bool,
    #[serde(default)]
    pub is_system_detected: bool,
}

/// Job details response with related data
#[derive(Debug, Deserialize, Clone)]
pub struct JobDetails {
    pub job: Job,
    pub emails: Vec<serde_json::Value>,
    pub comments: Vec<serde_json::Value>,
    pub timeline_events: Vec<serde_json::Value>,
    pub people: Vec<serde_json::Value>,
    #[serde(default)]
    pub contacts: Vec<EmailContact>,
}
