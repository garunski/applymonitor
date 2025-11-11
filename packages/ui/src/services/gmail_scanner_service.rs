//! Gmail Scanner API service

use crate::services::{error::ServiceError, http_client};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

/// Scan request payload
#[derive(Debug, Serialize)]
pub struct ScanRequest {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Scan response
#[derive(Debug, Deserialize)]
pub struct ScanResponse {
    pub scan_id: String,
    pub emails_found: usize,
    pub messages: Vec<GmailMessage>,
}

/// Gmail message structure
#[derive(Debug, Deserialize, Clone)]
pub struct GmailMessage {
    pub id: String,
    pub thread_id: String,
    pub snippet: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
}

/// Gmail Scanner API service
pub struct GmailScannerService;

impl GmailScannerService {
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

    /// Trigger a Gmail scan for the last 1 day by default
    pub async fn scan_emails(
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Result<ScanResponse, ServiceError> {
        let end_date = end_date.unwrap_or_else(|| Utc::now().to_rfc3339());
        let start_date =
            start_date.unwrap_or_else(|| (Utc::now() - Duration::days(1)).to_rfc3339());

        let url = format!("{}/scan", Self::get_scanner_base_url());
        let request = ScanRequest {
            start_date: Some(start_date),
            end_date: Some(end_date),
        };

        let body = serde_json::to_string(&request)
            .map_err(|e| ServiceError::Parse(format!("Failed to serialize request: {}", e)))?;

        let response = http_client::post(&url, Some(&body)).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<ScanResponse>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Get scan status
    pub async fn get_scan_status() -> Result<GmailStatus, ServiceError> {
        let url = format!("{}/status", Self::get_scanner_base_url());

        let response = http_client::get(&url).await?;
        let status = response.status();

        if status == 200 {
            http_client::json::<GmailStatus>(response).await
        } else if status == 401 {
            Err(ServiceError::Unauthorized)
        } else {
            let text = http_client::text(response).await.unwrap_or_default();
            Err(ServiceError::Server(status, text))
        }
    }

    /// Get Gmail OAuth URL for connecting Gmail
    pub fn get_gmail_auth_url() -> String {
        format!("{}/auth", Self::get_scanner_base_url())
    }
}

/// Gmail connection status
#[derive(Debug, Deserialize)]
pub struct GmailStatus {
    pub connected: bool,
    pub expires_at: Option<String>,
}
