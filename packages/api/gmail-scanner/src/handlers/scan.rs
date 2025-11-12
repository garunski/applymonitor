use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::common::uuid;
use crate::services::gmail_api;
use crate::services::gmail_oauth;
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use serde_json::Value;
use worker::*;

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub async fn scan_emails(mut req: Request, env: Env) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let body: ScanRequest = req.json().await?;

    let end_date = body
        .end_date
        .as_ref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);

    let start_date = body
        .start_date
        .as_ref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| end_date - Duration::days(7));

    if start_date >= end_date {
        return Response::error("start_date must be before end_date", 400);
    }

    let max_days = Duration::days(90);
    if end_date - start_date > max_days {
        return Response::error("Date range cannot exceed 90 days", 400);
    }

    let db = get_d1(&env)?;

    let token_row = db
        .prepare(
            "SELECT access_token, refresh_token, expires_at FROM gmail_tokens WHERE user_id = ?",
        )
        .bind(&[user_id.clone().into()])?
        .first::<Value>(None)
        .await?;

    let (access_token, _refresh_token) = if let Some(row) = token_row {
        let expires_at_str = row
            .get("expires_at")
            .and_then(|v| v.as_str())
            .ok_or_else(|| worker::Error::RustError("Missing expires_at".to_string()))?;

        let expires_at = DateTime::parse_from_rfc3339(expires_at_str)
            .map_err(|e| worker::Error::RustError(format!("Invalid expires_at: {}", e)))?
            .with_timezone(&Utc);

        let stored_access_token = row
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| worker::Error::RustError("Missing access_token".to_string()))?
            .to_string();

        let stored_refresh_token = row
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| worker::Error::RustError("Missing refresh_token".to_string()))?
            .to_string();

        if expires_at <= Utc::now() {
            let client_id = env
                .secret("GMAIL_CLIENT_ID")
                .map_err(|_| {
                    worker::Error::RustError("GMAIL_CLIENT_ID secret not found".to_string())
                })?
                .to_string();

            let client_secret = env
                .secret("GMAIL_CLIENT_SECRET")
                .map_err(|_| {
                    worker::Error::RustError("GMAIL_CLIENT_SECRET secret not found".to_string())
                })?
                .to_string();

            let token_response =
                gmail_oauth::refresh_token(&stored_refresh_token, &client_id, &client_secret)
                    .await
                    .map_err(|e| {
                        worker::Error::RustError(format!("Token refresh failed: {}", e))
                    })?;

            let new_expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

            db.prepare(
                "UPDATE gmail_tokens SET access_token = ?, expires_at = ?, updated_at = CURRENT_TIMESTAMP WHERE user_id = ?",
            )
            .bind(&[
                token_response.access_token.clone().into(),
                new_expires_at.to_rfc3339().into(),
                user_id.clone().into(),
            ])?
            .run()
            .await?;

            (token_response.access_token, stored_refresh_token)
        } else {
            (stored_access_token, stored_refresh_token)
        }
    } else {
        return Response::error("Gmail not connected", 401);
    };

    let scan_id = uuid::generate_uuid()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate UUID: {}", e)))?;

    db.prepare(
        "INSERT INTO email_scans (id, user_id, start_date, end_date, status) VALUES (?, ?, ?, ?, 'pending')",
    )
    .bind(&[
        scan_id.clone().into(),
        user_id.clone().into(),
        start_date.to_rfc3339().into(),
        end_date.to_rfc3339().into(),
    ])?
    .run()
    .await?;

    let query = gmail_api::build_date_query(&start_date, &end_date);
    let mut all_messages = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let response = gmail_api::list_messages(
            &access_token,
            Some(&query),
            Some(100),
            page_token.as_deref(),
        )
        .await
        .map_err(|e| worker::Error::RustError(format!("Gmail API error: {}", e)))?;

        if let Some(messages) = response.messages {
            for msg_item in messages {
                match gmail_api::get_message(&access_token, &msg_item.id).await {
                    Ok(msg) => all_messages.push(msg),
                    Err(e) => {
                        console_log!("Error fetching message {}: {}", msg_item.id, e);
                    }
                }
            }
        }

        page_token = response.next_page_token;
        if page_token.is_none() {
            break;
        }
    }

    // Store emails in database
    let mut stored_count = 0;
    for msg in &all_messages {
        // Parse date if available
        let date_str = msg.date.as_ref().and_then(|d| {
            DateTime::parse_from_rfc3339(d)
                .ok()
                .or_else(|| {
                    // Try other date formats
                    chrono::DateTime::parse_from_str(d, "%a, %d %b %Y %H:%M:%S %z").ok()
                })
                .map(|dt| dt.with_timezone(&Utc).to_rfc3339())
        });

        match db
            .prepare(
                "INSERT OR IGNORE INTO emails (gmail_id, user_id, scan_id, thread_id, subject, \"from\", \"to\", snippet, date) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&[
                msg.id.clone().into(),
                user_id.clone().into(),
                scan_id.clone().into(),
                msg.thread_id.clone().into(),
                msg.subject.as_deref().into(),
                msg.from.as_deref().into(),
                msg.to.as_deref().into(),
                msg.snippet.clone().into(),
                date_str.as_deref().into(),
            ])?
            .run()
            .await
        {
            Ok(_) => stored_count += 1,
            Err(e) => {
                console_log!("Error storing email {}: {}", msg.id, e);
            }
        }
    }

    let emails_found = all_messages.len() as i32;

    db.prepare(
        "UPDATE email_scans SET status = 'completed', emails_found = ?, completed_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(&[
        emails_found.into(),
        scan_id.clone().into(),
    ])?
    .run()
    .await?;

    Response::from_json(&serde_json::json!({
        "scan_id": scan_id,
        "emails_found": all_messages.len(),
        "messages": all_messages,
        "stored_count": stored_count
    }))
}

pub async fn get_scan(req: Request, env: Env, scan_id: String) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&env)?;
    let result = db
        .prepare("SELECT * FROM email_scans WHERE id = ? AND user_id = ?")
        .bind(&[scan_id.into(), user_id.into()])?
        .first::<Value>(None)
        .await?;

    match result {
        Some(scan) => Response::from_json(&scan),
        None => Response::error("Scan not found", 404),
    }
}

pub async fn list_scans(req: Request, env: Env) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&env)?;
    let result = db
        .prepare("SELECT * FROM email_scans WHERE user_id = ? ORDER BY created_at DESC LIMIT 50")
        .bind(&[user_id.into()])?
        .all()
        .await?;

    let scans: Vec<Value> = result.results()?;
    Response::from_json(&scans)
}
