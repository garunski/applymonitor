use anyhow::{anyhow, Result};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
struct SessionClaims {}

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

    let key = HS256Key::from_bytes(signing_key.as_bytes());
    let claims = key
        .verify_token::<SessionClaims>(&session_cookie, None)
        .map_err(|e| anyhow!("Invalid session token: {}", e))?;

    let user_id = claims
        .subject
        .ok_or_else(|| anyhow!("Token missing subject (sub) claim"))?;

    Ok(user_id)
}
