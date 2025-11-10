use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::unlink_provider;
use crate::services::oidc::OIDCProvider;
use crate::services::password;
use std::collections::HashMap;
use worker::*;

use super::helpers::{get_api_base_url, is_secure_cookie};

/// Link provider to existing account
pub async fn link_provider_endpoint(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let user_id = require_auth(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let url = req.url()?;
    let query_params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let default_provider = "google".to_string();
    let provider = query_params.get("provider").unwrap_or(&default_provider);

    if provider != "google" {
        return Response::error("Unsupported provider", 400);
    }

    // Store user_id in cookie for linking mode
    let secure_flag = if is_secure_cookie(&ctx.env) {
        "Secure; "
    } else {
        ""
    };

    let user_id_cookie = format!(
        "oauth_user_id={}; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=600",
        user_id, secure_flag
    );

    let headers = Headers::new();
    headers.set("Set-Cookie", &user_id_cookie)?;

    // Generate state and nonce
    let state = password::generate_secure_token()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate state: {}", e)))?;
    let nonce = password::generate_secure_token()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate nonce: {}", e)))?;

    let oidc = OIDCProvider::discover(&ctx.env, "https://accounts.google.com")
        .await
        .map_err(|e| {
            worker::Error::RustError(format!("Failed to discover OIDC provider: {}", e))
        })?;

    let redirect_uri = format!("{}/auth/callback", get_api_base_url(&ctx.env)?);
    let auth_url = oidc.build_authorization_url(&redirect_uri, &state, &nonce);

    // Set OAuth cookies
    let state_cookie = format!(
        "oauth_state={}; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=600",
        state, secure_flag
    );
    let nonce_cookie = format!(
        "oauth_nonce={}; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=600",
        nonce, secure_flag
    );
    let provider_cookie = format!(
        "oauth_provider={}; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=600",
        provider, secure_flag
    );

    headers.append("Set-Cookie", &state_cookie)?;
    headers.append("Set-Cookie", &nonce_cookie)?;
    headers.append("Set-Cookie", &provider_cookie)?;

    // Redirect directly to Google OAuth instead of returning JSON
    // This ensures cookies are set via navigation request, which works across origins
    headers.set("Location", &auth_url)?;

    let response = Response::ok("")?.with_headers(headers).with_status(302);

    Ok(response)
}

/// Unlink provider from account
pub async fn unlink_provider_endpoint(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let user_id = require_auth(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let url = req.url()?;
    let query_params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let default_provider = "google".to_string();
    let provider = query_params.get("provider").unwrap_or(&default_provider);

    let db = get_d1(&ctx.env)?;
    unlink_provider(&db, &user_id, provider)
        .await
        .map_err(|e| {
            let error_msg = format!("{}", e);
            if error_msg.contains("Cannot unlink last provider") {
                worker::Error::RustError("Cannot unlink last provider".to_string())
            } else {
                worker::Error::RustError(format!("Failed to unlink provider: {}", e))
            }
        })?;

    Response::ok("Provider unlinked successfully")
}
