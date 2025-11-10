use crate::services::session;
use anyhow::{anyhow, Result};
use worker::*;

pub fn get_session_cookie(req: &Request) -> Option<String> {
    let headers = req.headers();
    let cookie_header = headers.get("Cookie").ok()??;

    console_log!("[Auth] Cookie header received: {}", cookie_header);

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(stripped) = cookie.strip_prefix("session=") {
            console_log!("[Auth] Session cookie found, length: {}", stripped.len());
            return Some(stripped.to_string());
        }
    }

    console_log!("[Auth] No session cookie found in header");
    None
}

pub async fn require_auth(req: &Request, env: &Env) -> Result<String> {
    let session_cookie =
        get_session_cookie(req).ok_or_else(|| {
            console_log!("[Auth] require_auth: No session cookie found");
            anyhow!("No session cookie found")
        })?;

    console_log!("[Auth] require_auth: Session cookie found, verifying token...");

    let signing_key = env
        .secret("SESSION_SIGNING_KEY")
        .map_err(|_| anyhow!("SESSION_SIGNING_KEY secret not found"))?
        .to_string();

    let user_id = session::verify_session_token(&session_cookie, &signing_key)
        .map_err(|e| {
            console_log!("[Auth] require_auth: Token verification failed: {}", e);
            anyhow!("Invalid session token: {}", e)
        })?;

    console_log!("[Auth] require_auth: Token verified successfully, user_id: {}", user_id);
    Ok(user_id)
}
