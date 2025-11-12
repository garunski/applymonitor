//! Email contacts API service

use crate::services::jobs_service::EmailContact;
use crate::services::{error::ServiceError, http_client};
use serde::Serialize;

/// Request for updating a contact
#[derive(Debug, Serialize)]
pub struct UpdateContactRequest {
    pub name: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
}

/// Email contacts API service
pub struct EmailContactsService;

impl EmailContactsService {
    /// Get the base URL for the API
    fn get_api_base_url() -> String {
        crate::services::api_config::get_api_base_url()
    }

    /// Get contacts for a job
    pub async fn get_contacts_for_job(job_id: &str) -> Result<Vec<EmailContact>, ServiceError> {
        let url = format!(
            "{}/email-contacts?job_id={}",
            Self::get_api_base_url(),
            job_id
        );

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<Vec<EmailContact>>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Get a specific contact by email
    pub async fn get_contact(email: &str) -> Result<EmailContact, ServiceError> {
        let encoded = email.replace('@', "%40");
        let url = format!("{}/email-contacts/{}", Self::get_api_base_url(), encoded);

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<EmailContact>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Update a contact
    pub async fn update_contact(
        email: &str,
        name: Option<String>,
        linkedin: Option<String>,
        website: Option<String>,
    ) -> Result<EmailContact, ServiceError> {
        let encoded = email.replace('@', "%40");
        let url = format!("{}/email-contacts/{}", Self::get_api_base_url(), encoded);

        let body = UpdateContactRequest {
            name,
            linkedin,
            website,
        };
        let body_str = serde_json::to_string(&body)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize: {}", e)))?;

        let response = http_client::put(&url, Some(&body_str)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<EmailContact>(response).await
        } else if status == 404 {
            Err(ServiceError::NotFound)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Check if an email is detected as a system email
    pub async fn check_system_email(email: &str) -> Result<bool, ServiceError> {
        let encoded = email.replace('@', "%40");
        let url = format!(
            "{}/email-contacts/{}?check_system=true",
            Self::get_api_base_url(),
            encoded
        );

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            #[derive(serde::Deserialize)]
            struct CheckResponse {
                is_system_detected: bool,
            }
            let check_response: CheckResponse = http_client::json(response).await?;
            Ok(check_response.is_system_detected)
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Convert a system email contact to a user-saved contact
    pub async fn convert_to_user_contact(email: &str) -> Result<EmailContact, ServiceError> {
        let encoded = email.replace('@', "%40");
        let url = format!(
            "{}/email-contacts/{}?action=convert-to-user",
            Self::get_api_base_url(),
            encoded
        );

        let response = http_client::post(&url, None).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<EmailContact>(response).await
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
