use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use worker::*;

#[derive(Debug, Serialize)]
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

pub async fn get_ai_result(db: &D1Database, email_id: &str) -> Result<Option<AiResult>> {
    let result = db
        .prepare("SELECT id, email_id, user_id, category, confidence, company, job_title, summary, extracted_data, created_at FROM ai_results WHERE email_id = ? LIMIT 1")
        .bind(&[email_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        Ok(Some(AiResult {
            id: row
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            email_id: row
                .get("email_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            user_id: row
                .get("user_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            category: row
                .get("category")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            confidence: row.get("confidence").and_then(|v| v.as_f64()),
            company: row
                .get("company")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            job_title: row
                .get("job_title")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            summary: row
                .get("summary")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            extracted_data: row
                .get("extracted_data")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: row
                .get("created_at")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        }))
    } else {
        Ok(None)
    }
}
