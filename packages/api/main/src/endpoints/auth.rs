use crate::common::db::get_d1;
use crate::services::db::{get_user_by_id, upsert_user};
use crate::services::oidc::OIDCProvider;
use crate::services::session;
use serde_json::json;
use std::collections::HashMap;
use worker::*;

pub async fn login(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let query_params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let default_provider = "google".to_string();
    let provider = query_params.get("provider").unwrap_or(&default_provider);

    if provider != "google" {
        return Response::error("Unsupported provider", 400);
    }

    // Generate state and nonce for CSRF protection
    let state = generate_random_string(32);
    let nonce = generate_random_string(32);

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

    // Set state and nonce cookies
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

    // Return JSON response with OAuth URL and set cookies
    let headers = Headers::new();
    headers.set("Set-Cookie", &state_cookie)?;
    headers.append("Set-Cookie", &nonce_cookie)?;

    let response_json = json!({
        "auth_url": auth_url
    });

    let response = Response::from_json(&response_json)?.with_headers(headers);

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

    // Upsert user in database
    let db = get_d1(&ctx.env)?;
    let user_id = upsert_user(
        &db,
        "google",
        &claims.sub,
        claims.email.as_deref(),
        claims.name.as_deref(),
        claims.picture.as_deref(),
    )
    .await
    .map_err(|e| worker::Error::RustError(format!("Failed to upsert user: {}", e)))?;

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

    let session_token = session::make_session_token(user_id, &signing_key, &jwt_issuer)
        .map_err(|e| worker::Error::RustError(format!("Failed to create session token: {}", e)))?;

    // Get frontend URL for redirect
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
            "{}={}; HttpOnly; {}SameSite=Strict; Path=/; Max-Age=86400",
            cookie_name, session_token, secure_flag
        ),
    )?;
    // Clear OAuth state cookies
    headers.append(
        "Set-Cookie",
        &format!(
            "oauth_state=; HttpOnly; {}SameSite=Strict; Path=/; Max-Age=0",
            secure_flag
        ),
    )?;
    headers.append(
        "Set-Cookie",
        &format!(
            "oauth_nonce=; HttpOnly; {}SameSite=Strict; Path=/; Max-Age=0",
            secure_flag
        ),
    )?;

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

pub async fn me(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    use crate::common::auth::require_auth;

    // Note: Auth is already checked in lib.rs before routing, but we check again here
    // for safety. If auth fails here, it means the check in lib.rs was bypassed somehow.
    let user_id = match require_auth(&req, &ctx.env).await {
        Ok(id) => id,
        Err(e) => {
            // Return error (CORS will be applied globally)
            return Response::error(format!("Unauthorized: {}", e), 401);
        }
    };

    let db = get_d1(&ctx.env)?;
    let user = get_user_by_id(&db, user_id)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get user: {}", e)))?;

    if let Some(user) = user {
        let user_json = json!({
            "id": user.id,
            "provider": user.provider,
            "email": user.email,
            "name": user.name,
            "avatar": user.avatar,
            "created_at": user.created_at,
            "last_login": user.last_login,
        });

        Response::from_json(&user_json)
    } else {
        Response::error("User not found", 404)
    }
}

// Helper functions
fn generate_random_string(length: usize) -> String {
    use js_sys::Date;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Generate multiple hashes to ensure we have enough length
    let mut hasher = DefaultHasher::new();
    let timestamp = Date::now() as u64;
    timestamp.hash(&mut hasher);
    let hash1 = hasher.finish();

    let mut hasher2 = DefaultHasher::new();
    hash1.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    // Concatenate both hashes to get a longer string
    let combined = format!("{:x}{:x}", hash1, hash2);
    // Take the requested length (combined is 32 hex chars, enough for most cases)
    combined.chars().take(length).collect()
}

fn extract_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with(&format!("{}=", name)) {
            return Some(cookie[name.len() + 1..].to_string());
        }
    }
    None
}

fn get_api_base_url(env: &Env) -> worker::Result<String> {
    // Try to get from environment variable or use default
    if let Ok(url) = env.var("API_BASE_URL") {
        return Ok(url.to_string());
    }

    // Default to production URL
    Ok("https://api.applymonitor.com".to_string())
}

fn is_secure_cookie(env: &Env) -> bool {
    // Check if API_BASE_URL starts with https://
    if let Ok(url) = env.var("API_BASE_URL") {
        return url.to_string().starts_with("https://");
    }
    // Default to secure for production
    true
}
