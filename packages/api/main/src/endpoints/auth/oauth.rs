use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::{find_or_create_user, link_provider};
use crate::services::oidc::OIDCProvider;
use crate::services::password;
use crate::services::session;
use std::collections::HashMap;
use worker::*;

use super::helpers::{extract_cookie_value, get_api_base_url, is_secure_cookie};

pub async fn login(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let query_params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let default_provider = "google".to_string();
    let provider = query_params.get("provider").unwrap_or(&default_provider);

    if provider != "google" {
        return Response::error("Unsupported provider", 400);
    }

    // Generate state and nonce for CSRF protection using secure random
    let state = password::generate_secure_token()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate state: {}", e)))?;
    let nonce = password::generate_secure_token()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate nonce: {}", e)))?;

    let oidc = OIDCProvider::discover(&ctx.env, "https://accounts.google.com")
        .await
        .map_err(|e| {
            // Return a more user-friendly error if secrets are missing
            let error_msg = format!("{}", e);
            if error_msg.contains("secret not found") {
                worker::Error::RustError(
                    "OIDC provider not configured. Please configure OIDC_GOOGLE_CLIENT_ID and OIDC_GOOGLE_CLIENT_SECRET secrets.".to_string()
                )
            } else {
                worker::Error::RustError(format!("Failed to discover OIDC provider: {}", e))
            }
        })?;

    let redirect_uri = format!("{}/auth/callback", get_api_base_url(&ctx.env)?);
    let auth_url = oidc.build_authorization_url(&redirect_uri, &state, &nonce);

    // Check for account linking mode (oauth_user_id cookie)
    let cookie_header = req
        .headers()
        .get("Cookie")
        .unwrap_or_default()
        .unwrap_or_default();
    let linking_user_id = extract_cookie_value(&cookie_header, "oauth_user_id");

    // Set state, nonce, and provider cookies
    let secure_flag = if is_secure_cookie(&ctx.env) {
        "Secure; "
    } else {
        ""
    };
    // Use SameSite=Lax for OAuth cookies to allow cross-site redirects from OAuth provider
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

    // Set cookies and redirect directly to Google OAuth (not JSON response)
    // This ensures cookies are set via navigation request, not fetch request
    let headers = Headers::new();
    headers.set("Location", &auth_url)?;
    headers.set("Set-Cookie", &state_cookie)?;
    headers.append("Set-Cookie", &nonce_cookie)?;
    headers.append("Set-Cookie", &provider_cookie)?;

    // If linking mode, keep oauth_user_id cookie
    if let Some(user_id) = linking_user_id {
        let user_id_cookie = format!(
            "oauth_user_id={}; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=600",
            user_id, secure_flag
        );
        headers.append("Set-Cookie", &user_id_cookie)?;
    }

    // Redirect directly to Google OAuth instead of returning JSON
    // This ensures cookies are set via navigation request, which works across origins
    let response = Response::ok("")?.with_headers(headers).with_status(302);

    Ok(response)
}

pub async fn callback(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let query_params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let code = query_params
        .get("code")
        .ok_or_else(|| worker::Error::RustError("Missing authorization code".to_string()))?;

    let state = query_params
        .get("state")
        .ok_or_else(|| worker::Error::RustError("Missing state parameter".to_string()))?;

    // Get state from cookie
    let cookie_header = req
        .headers()
        .get("Cookie")
        .unwrap_or_default()
        .unwrap_or_default();
    let cookie_state = extract_cookie_value(&cookie_header, "oauth_state");
    let cookie_nonce = extract_cookie_value(&cookie_header, "oauth_nonce");
    let cookie_provider = extract_cookie_value(&cookie_header, "oauth_provider");
    let linking_user_id = extract_cookie_value(&cookie_header, "oauth_user_id");

    // Debug: log cookie state for troubleshooting
    // In production, you might want to remove this or make it conditional
    if cookie_state.is_none() {
        return Response::error(
            format!("No state cookie found. Cookie header: {}", cookie_header),
            400,
        );
    }

    if cookie_state.as_deref() != Some(state.as_str()) {
        return Response::error(
            format!(
                "State mismatch. Expected: {}, Got: {}, Cookie header: {}",
                state,
                cookie_state.as_deref().unwrap_or("none"),
                cookie_header
            ),
            400,
        );
    }

    let nonce =
        cookie_nonce.ok_or_else(|| worker::Error::RustError("Missing nonce cookie".to_string()))?;

    // Exchange code for tokens
    let oidc = OIDCProvider::discover(&ctx.env, "https://accounts.google.com")
        .await
        .map_err(|e| {
            worker::Error::RustError(format!("Failed to discover OIDC provider: {}", e))
        })?;

    let redirect_uri = format!("{}/auth/callback", get_api_base_url(&ctx.env)?);
    let token_response = oidc
        .exchange_code(code, &redirect_uri)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to exchange code: {}", e)))?;

    // Validate ID token
    let claims = oidc
        .validate_id_token(&token_response.id_token, &nonce)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to validate ID token: {}", e)))?;

    // Get provider from cookie or default to google
    let provider = cookie_provider.as_deref().unwrap_or("google");

    // Find or create user with account linking support
    let db = get_d1(&ctx.env)?;
    let user_id = if let Some(ref linking_id) = linking_user_id {
        // Account linking mode - verify session exists and matches
        let linking_user_id_str = linking_id.to_string();

        // Try to get session user_id (may not exist yet if this is the first login)
        // For account linking, we require an existing session
        let session_user_id = require_auth(&req, &ctx.env).await.map_err(|_| {
            worker::Error::RustError(
                "Authentication required for account linking. Please sign in first.".to_string(),
            )
        })?;

        if session_user_id != linking_user_id_str {
            return Response::error("Session user_id mismatch", 403);
        }

        // Check if provider already linked
        match link_provider(&db, &linking_user_id_str, provider, &claims.sub).await {
            Ok(_) => linking_user_id_str,
            Err(e) => {
                let error_msg = format!("{}", e);
                if error_msg.contains("already linked to another account") {
                    return Response::error("Provider already linked to another account", 409);
                }
                return Response::error(format!("Failed to link provider: {}", e), 500);
            }
        }
    } else {
        // Normal login flow
        find_or_create_user(
            &db,
            provider,
            &claims.sub,
            claims.email.as_deref(),
            claims.name.as_deref(),
            claims.picture.as_deref(),
        )
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to find or create user: {}", e)))?
    };

    // Create session token
    let signing_key = ctx
        .env
        .secret("SESSION_SIGNING_KEY")
        .map_err(|_| worker::Error::RustError("SESSION_SIGNING_KEY secret not found".to_string()))?
        .to_string();

    let jwt_issuer = ctx
        .env
        .var("JWT_ISSUER")
        .map_err(|_| worker::Error::RustError("JWT_ISSUER not found".to_string()))?
        .to_string();

    let session_token = session::make_session_token(&user_id, &signing_key, &jwt_issuer)
        .map_err(|e| worker::Error::RustError(format!("Failed to create session token: {}", e)))?;

    console_log!("[OAuth callback] Session created for user_id: {}", user_id);
    console_log!(
        "[OAuth callback] Session token length: {}",
        session_token.len()
    );

    // Get frontend URL for redirect (different for account linking)
    let frontend_url = if linking_user_id.is_some() {
        ctx.env
            .var("FRONTEND_URL")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| "https://applymonitor.com".to_string())
            + "/settings/accounts"
    } else {
        ctx.env
            .var("FRONTEND_URL")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| "https://applymonitor.com".to_string())
            + "/dashboard"
    };

    let cookie_name = ctx
        .env
        .var("SESSION_COOKIE_NAME")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "session".to_string());

    console_log!("[OAuth callback] Cookie name: {}", cookie_name);
    console_log!("[OAuth callback] Redirecting to frontend: {}", frontend_url);
    console_log!(
        "[OAuth callback] Secure flag: {}",
        if is_secure_cookie(&ctx.env) {
            "Secure"
        } else {
            "Not secure"
        }
    );

    // Create response with headers set from the start (headers are immutable after creation)
    let headers = Headers::new();
    headers.set("Location", &frontend_url)?;

    let secure_flag = if is_secure_cookie(&ctx.env) {
        "Secure; "
    } else {
        ""
    };
    let cookie_value = format!(
        "{}={}; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=86400",
        cookie_name, session_token, secure_flag
    );
    console_log!(
        "[OAuth callback] Set-Cookie header: {}={}... (truncated)",
        cookie_name,
        &session_token[..session_token.len().min(20)]
    );
    headers.set("Set-Cookie", &cookie_value)?;
    // Clear OAuth state cookies
    headers.append(
        "Set-Cookie",
        &format!(
            "oauth_state=; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=0",
            secure_flag
        ),
    )?;
    headers.append(
        "Set-Cookie",
        &format!(
            "oauth_nonce=; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=0",
            secure_flag
        ),
    )?;
    headers.append(
        "Set-Cookie",
        &format!(
            "oauth_provider=; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=0",
            secure_flag
        ),
    )?;
    if linking_user_id.is_some() {
        headers.append(
            "Set-Cookie",
            &format!(
                "oauth_user_id=; HttpOnly; {}SameSite=Lax; Path=/; Max-Age=0",
                secure_flag
            ),
        )?;
    }

    let response = Response::ok("")?.with_headers(headers).with_status(302);

    Ok(response)
}

pub async fn logout(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let frontend_url = ctx
        .env
        .var("FRONTEND_URL")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "https://applymonitor.com".to_string());

    let cookie_name = ctx
        .env
        .var("SESSION_COOKIE_NAME")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "session".to_string());

    // Create response with headers set from the start (headers are immutable after creation)
    let headers = Headers::new();
    headers.set("Location", &frontend_url)?;

    let secure_flag = if is_secure_cookie(&ctx.env) {
        "Secure; "
    } else {
        ""
    };
    headers.set(
        "Set-Cookie",
        &format!(
            "{}={}; HttpOnly; {}SameSite=Strict; Path=/; Max-Age=0",
            cookie_name, "", secure_flag
        ),
    )?;

    let response = Response::ok("")?.with_headers(headers).with_status(302);

    Ok(response)
}
