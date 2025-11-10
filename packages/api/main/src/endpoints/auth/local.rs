use crate::common::db::get_d1;
use crate::services::db::{
    create_local_user, get_password_hash, get_user_by_email, get_user_by_id, update_password,
};
use crate::services::password;
use crate::services::session;
use serde_json::json;
use worker::*;

use super::helpers::is_secure_cookie;

/// Register new user with local authentication
pub async fn register(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    #[derive(serde::Deserialize)]
    struct RegisterRequest {
        email: String,
        password: String,
        name: String,
    }

    let register_data: RegisterRequest = req.json().await?;

    // Validate email format
    if !register_data.email.contains('@') {
        return Response::error("Invalid email format", 400);
    }

    // Validate password strength
    if register_data.password.len() < 8 {
        return Response::error("Password must be at least 8 characters", 400);
    }

    // Validate name
    if register_data.name.trim().is_empty() {
        return Response::error("Name is required", 400);
    }

    // Hash password
    let password_hash = password::hash_password(&register_data.password)
        .map_err(|e| worker::Error::RustError(format!("Failed to hash password: {}", e)))?;

    // Create user
    let db = get_d1(&ctx.env)?;
    let user_id = create_local_user(
        &db,
        &register_data.email,
        &register_data.name,
        &password_hash,
    )
    .await
    .map_err(|e| {
        let error_msg = format!("{}", e);
        if error_msg.contains("already registered") {
            worker::Error::RustError("Email already registered".to_string())
        } else {
            worker::Error::RustError(format!("Failed to create user: {}", e))
        }
    })?;

    // Create session
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

    let cookie_name = ctx
        .env
        .var("SESSION_COOKIE_NAME")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "session".to_string());

    let secure_flag = if is_secure_cookie(&ctx.env) {
        "Secure; "
    } else {
        ""
    };

    let headers = Headers::new();
    headers.set(
        "Set-Cookie",
        &format!(
            "{}={}; HttpOnly; {}SameSite=Strict; Path=/; Max-Age=86400",
            cookie_name, session_token, secure_flag
        ),
    )?;

    // Get user for response
    let user = get_user_by_id(&db, &user_id)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get user: {}", e)))?;

    if let Some(user) = user {
        let user_json = json!({
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "picture": user.picture,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
            "providers": user.providers,
        });

        Ok(Response::from_json(&user_json)?.with_headers(headers))
    } else {
        Response::error("User not found", 404)
    }
}

/// Login with local username/password
pub async fn login_local(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    #[derive(serde::Deserialize)]
    struct LoginRequest {
        email: String,
        password: String,
    }

    let login_data: LoginRequest = req.json().await?;

    // Get user by email
    let db = get_d1(&ctx.env)?;
    let user = get_user_by_email(&db, &login_data.email)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get user: {}", e)))?;

    let user = user.ok_or_else(|| worker::Error::RustError("Invalid credentials".to_string()))?;

    // Check if user has local provider
    if !user.providers.contains(&"local".to_string()) {
        return Response::error("Invalid credentials", 401);
    }

    // Get password hash
    let password_hash = get_password_hash(&db, &user.id)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get password hash: {}", e)))?;

    let password_hash =
        password_hash.ok_or_else(|| worker::Error::RustError("Invalid credentials".to_string()))?;

    // Verify password
    let is_valid = password::verify_password(&login_data.password, &password_hash)
        .map_err(|e| worker::Error::RustError(format!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Response::error("Invalid credentials", 401);
    }

    // Check if rehash needed
    if password::needs_rehash(&password_hash) {
        let new_hash = password::hash_password(&login_data.password)
            .map_err(|e| worker::Error::RustError(format!("Failed to hash password: {}", e)))?;
        update_password(&db, &user.id, &new_hash)
            .await
            .map_err(|e| worker::Error::RustError(format!("Failed to update password: {}", e)))?;
    }

    // Create session
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

    let session_token = session::make_session_token(&user.id, &signing_key, &jwt_issuer)
        .map_err(|e| worker::Error::RustError(format!("Failed to create session token: {}", e)))?;

    let cookie_name = ctx
        .env
        .var("SESSION_COOKIE_NAME")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "session".to_string());

    let secure_flag = if is_secure_cookie(&ctx.env) {
        "Secure; "
    } else {
        ""
    };

    let headers = Headers::new();
    headers.set(
        "Set-Cookie",
        &format!(
            "{}={}; HttpOnly; {}SameSite=Strict; Path=/; Max-Age=86400",
            cookie_name, session_token, secure_flag
        ),
    )?;

    let user_json = json!({
        "id": user.id,
        "email": user.email,
        "name": user.name,
        "picture": user.picture,
        "created_at": user.created_at,
        "updated_at": user.updated_at,
        "providers": user.providers,
    });

    Ok(Response::from_json(&user_json)?.with_headers(headers))
}
