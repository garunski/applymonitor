//! Jobs API service

use crate::services::{api_config::get_api_base_url, error::ServiceError};
use serde::{Deserialize, Serialize};

/// Job struct matching API response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Job {
    pub id: Option<i64>,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
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

        match gloo_net::http::Request::get(&url).send().await {
            Ok(response) => {
                if response.status() == 200 {
                    match response.json::<Vec<Job>>().await {
                        Ok(jobs) => Ok(jobs),
                        Err(e) => Err(ServiceError::Parse(format!("Failed to parse jobs: {}", e))),
                    }
                } else if response.status() == 404 {
                    Err(ServiceError::NotFound)
                } else if response.status() == 401 {
                    Err(ServiceError::Unauthorized)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ServiceError::Server(status, text))
                }
            }
            Err(e) => Err(ServiceError::Network(format!(
                "Failed to fetch jobs: {}",
                e
            ))),
        }
    }

    /// Fetch a single job by ID
    pub async fn fetch_job(id: i64) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs/{}", get_api_base_url(), id);

        match gloo_net::http::Request::get(&url).send().await {
            Ok(response) => {
                if response.status() == 200 {
                    match response.json::<Job>().await {
                        Ok(job) => Ok(job),
                        Err(e) => Err(ServiceError::Parse(format!("Failed to parse job: {}", e))),
                    }
                } else if response.status() == 404 {
                    Err(ServiceError::NotFound)
                } else if response.status() == 401 {
                    Err(ServiceError::Unauthorized)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ServiceError::Server(status, text))
                }
            }
            Err(e) => Err(ServiceError::Network(format!("Failed to fetch job: {}", e))),
        }
    }

    /// Create a new job
    pub async fn create_job(job: CreateJobRequest) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs", get_api_base_url());

        match gloo_net::http::Request::post(&url)
            .json(&job)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize job: {}", e)))?
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == 201 || response.status() == 200 {
                    match response.json::<Job>().await {
                        Ok(job) => Ok(job),
                        Err(e) => Err(ServiceError::Parse(format!(
                            "Failed to parse created job: {}",
                            e
                        ))),
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
                "Failed to create job: {}",
                e
            ))),
        }
    }

    /// Update an existing job
    pub async fn update_job(id: i64, job: UpdateJobRequest) -> Result<Job, ServiceError> {
        let url = format!("{}/jobs/{}", get_api_base_url(), id);

        match gloo_net::http::Request::put(&url)
            .json(&job)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize job: {}", e)))?
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == 200 {
                    match response.json::<Job>().await {
                        Ok(job) => Ok(job),
                        Err(e) => Err(ServiceError::Parse(format!(
                            "Failed to parse updated job: {}",
                            e
                        ))),
                    }
                } else if response.status() == 404 {
                    Err(ServiceError::NotFound)
                } else if response.status() == 401 {
                    Err(ServiceError::Unauthorized)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ServiceError::Server(status, text))
                }
            }
            Err(e) => Err(ServiceError::Network(format!(
                "Failed to update job: {}",
                e
            ))),
        }
    }

    /// Delete a job
    pub async fn delete_job(id: i64) -> Result<(), ServiceError> {
        let url = format!("{}/jobs/{}", get_api_base_url(), id);

        match gloo_net::http::Request::delete(&url).send().await {
            Ok(response) => {
                if response.status() == 200 || response.status() == 204 {
                    Ok(())
                } else if response.status() == 404 {
                    Err(ServiceError::NotFound)
                } else if response.status() == 401 {
                    Err(ServiceError::Unauthorized)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ServiceError::Server(status, text))
                }
            }
            Err(e) => Err(ServiceError::Network(format!(
                "Failed to delete job: {}",
                e
            ))),
        }
    }
}
