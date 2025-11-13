//! AI API service

use crate::services::{api_config::get_api_base_url, error::ServiceError, http_client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiPrompt {
    pub id: String,
    pub name: String,
    pub stage: String,
    pub prompt: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiResult {
    pub id: String,
    pub email_id: String,
    pub user_id: String,
    pub category: Option<String>,
    pub confidence: Option<f64>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub summary: Option<String>,
    pub extracted_data: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
struct ProcessRequest {
    user_id: String,
}

#[derive(Debug, Serialize)]
struct CreatePromptRequest {
    name: String,
    stage: String,
    prompt: String,
}

#[derive(Debug, Serialize)]
struct ActivatePromptRequest {
    stage: String,
}

#[derive(Debug, Serialize)]
struct TestPromptRequest {
    email_ids: Vec<String>,
    user_id: String,
}

/// Get AI worker base URL
fn get_ai_worker_base_url() -> String {
    if let Ok(url) = std::env::var("AI_WORKER_URL") {
        if !url.is_empty() {
            return url;
        }
    }

    #[cfg(feature = "api-prod")]
    {
        return "https://applymonitor-ai-processor.workers.dev".to_string();
    }

    #[cfg(feature = "api-staging")]
    {
        return "https://applymonitor-ai-processor-staging.workers.dev".to_string();
    }

    "http://localhost:8002".to_string()
}

/// AI API service
pub struct AiService;

impl AiService {
    /// Process a single email through AI pipeline
    pub async fn process_email(email_id: &str, user_id: &str) -> Result<(), ServiceError> {
        let url = format!("{}/process/{}", get_ai_worker_base_url(), email_id);

        let request_body = ProcessRequest {
            user_id: user_id.to_string(),
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

    /// Get AI results for an email
    pub async fn get_ai_results(email_id: &str) -> Result<AiResult, ServiceError> {
        let url = format!("{}/api/emails/{}/ai-results", get_api_base_url(), email_id);

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<AiResult>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// List all prompts (admin only)
    pub async fn list_prompts(stage: Option<&str>) -> Result<Vec<AiPrompt>, ServiceError> {
        let mut url = format!("{}/api/admin/prompts", get_api_base_url());
        if let Some(s) = stage {
            url = format!("{}?stage={}", url, s);
        }

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Vec<AiPrompt>>(response).await
        } else if status == 401 || status == 403 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Create a new prompt version (admin only)
    pub async fn create_prompt(
        name: &str,
        stage: &str,
        prompt: &str,
    ) -> Result<String, ServiceError> {
        let url = format!("{}/api/admin/prompts", get_api_base_url());

        let request_body = CreatePromptRequest {
            name: name.to_string(),
            stage: stage.to_string(),
            prompt: prompt.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            #[derive(Deserialize)]
            struct CreateResponse {
                id: String,
            }
            let result: CreateResponse = http_client::json::<CreateResponse>(response).await?;
            Ok(result.id)
        } else if status == 401 || status == 403 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Activate a prompt (admin only)
    pub async fn activate_prompt(prompt_id: &str, stage: &str) -> Result<(), ServiceError> {
        let url = format!(
            "{}/api/admin/prompts/{}/activate",
            get_api_base_url(),
            prompt_id
        );

        let request_body = ActivatePromptRequest {
            stage: stage.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
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

    /// Test a prompt on sample emails (admin only)
    pub async fn test_prompt(
        email_ids: Vec<String>,
        user_id: &str,
    ) -> Result<serde_json::Value, ServiceError> {
        let url = format!("{}/api/admin/prompts/test", get_api_base_url());

        let request_body = TestPromptRequest {
            email_ids,
            user_id: user_id.to_string(),
        };

        let body = serde_json::to_string(&request_body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<serde_json::Value>(response).await
        } else if status == 401 || status == 403 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }
}
