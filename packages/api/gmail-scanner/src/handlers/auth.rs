use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::gmail_oauth;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{Duration, Utc};
use getrandom::getrandom;
use serde_json::Value;
use url::Url;
use worker::*;

pub async fn initiate_auth(req: Request, env: Env) -> worker::Result<Response> {
    let _user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let client_id = env
        .secret("GMAIL_CLIENT_ID")
        .map_err(|_| worker::Error::RustError("GMAIL_CLIENT_ID secret not found".to_string()))?
        .to_string();

    let redirect_uri = format!("{}/gmail/callback", get_base_url(&env)?);

    let mut state_bytes = [0u8; 32];
    getrandom(&mut state_bytes)
        .map_err(|e| worker::Error::RustError(format!("Failed to generate state: {}", e)))?;
    let state = general_purpose::STANDARD.encode(state_bytes);

    let auth_url = gmail_oauth::build_authorization_url(&client_id, &redirect_uri, &state)
        .map_err(|e| worker::Error::RustError(format!("Failed to build auth URL: {}", e)))?;

    Response::redirect(
        Url::parse(&auth_url)
            .map_err(|e| worker::Error::RustError(format!("Invalid URL: {}", e)))?,
    )
}

pub async fn callback(req: Request, env: Env) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let url = req.url()?;
    let query_params: std::collections::HashMap<String, String> =
        url.query_pairs().into_owned().collect();

    let code = query_params
        .get("code")
        .ok_or_else(|| worker::Error::RustError("Missing code parameter".to_string()))?;

    let client_id = env
        .secret("GMAIL_CLIENT_ID")
        .map_err(|_| worker::Error::RustError("GMAIL_CLIENT_ID secret not found".to_string()))?
        .to_string();

    let client_secret = env
        .secret("GMAIL_CLIENT_SECRET")
        .map_err(|_| worker::Error::RustError("GMAIL_CLIENT_SECRET secret not found".to_string()))?
        .to_string();

    let redirect_uri = format!("{}/gmail/callback", get_base_url(&env)?);

    let token_response =
        gmail_oauth::exchange_code(code, &client_id, &client_secret, &redirect_uri)
            .await
            .map_err(|e| worker::Error::RustError(format!("Token exchange failed: {}", e)))?;

    let db = get_d1(&env)?;
    let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

    db.prepare(
        "INSERT OR REPLACE INTO gmail_tokens (user_id, access_token, refresh_token, expires_at, updated_at) VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)",
    )
    .bind(&[
        user_id.into(),
        token_response.access_token.into(),
        token_response.refresh_token.into(),
        expires_at.to_rfc3339().into(),
    ])?
    .run()
    .await?;

    let frontend_url = env
        .var("FRONTEND_URL")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "https://applymonitor.com".to_string());

    Response::redirect(
        Url::parse(&format!("{}/settings/accounts", frontend_url))
            .map_err(|e| worker::Error::RustError(format!("Invalid URL: {}", e)))?,
    )
}

pub async fn disconnect(req: Request, env: Env) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&env)?;
    db.prepare("DELETE FROM gmail_tokens WHERE user_id = ?")
        .bind(&[user_id.into()])?
        .run()
        .await?;

    Response::from_json(&serde_json::json!({ "success": true }))
}

pub async fn status(req: Request, env: Env) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&env)?;
    let result = db
        .prepare("SELECT expires_at FROM gmail_tokens WHERE user_id = ?")
        .bind(&[user_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        let expires_at = row
            .get("expires_at")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let is_connected = expires_at
            .as_ref()
            .and_then(|e| chrono::DateTime::parse_from_rfc3339(e).ok())
            .map(|dt| dt > chrono::Utc::now())
            .unwrap_or(false);

        Response::from_json(&serde_json::json!({
            "connected": is_connected,
            "expires_at": expires_at
        }))
    } else {
        Response::from_json(&serde_json::json!({ "connected": false }))
    }
}

fn get_base_url(env: &Env) -> worker::Result<String> {
    Ok(format!(
        "https://{}.workers.dev",
        env.var("WRANGLER_WORKER_NAME")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| "applymonitor-gmail-scanner".to_string())
    ))
}
