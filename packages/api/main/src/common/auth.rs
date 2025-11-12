use crate::common::db::get_d1;
use crate::services::db::is_user_admin;
use crate::services::session;
use anyhow::{anyhow, Result};
use worker::*;

pub fn get_session_cookie(req: &Request) -> Option<String> {
    let headers = req.headers();
    let cookie_header = headers.get("Cookie").ok()??;

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(stripped) = cookie.strip_prefix("session=") {
            return Some(stripped.to_string());
        }
    }

    None
}

pub async fn require_auth(req: &Request, env: &Env) -> Result<String> {
    let session_cookie =
        get_session_cookie(req).ok_or_else(|| anyhow!("No session cookie found"))?;

    let signing_key = env
        .secret("SESSION_SIGNING_KEY")
        .map_err(|_| anyhow!("SESSION_SIGNING_KEY secret not found"))?
        .to_string();

    let user_id = session::verify_session_token(&session_cookie, &signing_key)
        .map_err(|e| anyhow!("Invalid session token: {}", e))?;

    Ok(user_id)
}

pub async fn require_admin(req: &Request, env: &Env) -> Result<String> {
    let user_id = require_auth(req, env).await?;

    let db = get_d1(env)?;
    let is_admin = is_user_admin(&db, &user_id)
        .await
        .map_err(|e| anyhow!("Failed to check admin status: {}", e))?;

    if !is_admin {
        return Err(anyhow!("Admin access required"));
    }

    Ok(user_id)
}
