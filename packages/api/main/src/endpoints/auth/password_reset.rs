use crate::common::db::get_d1;
use crate::services::db::{
    create_password_reset_token, get_user_by_email, update_password, validate_password_reset_token,
};
use crate::services::password;
use worker::*;

/// Request password reset
pub async fn request_password_reset(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    #[derive(serde::Deserialize)]
    struct ResetRequest {
        email: String,
    }

    let reset_data: ResetRequest = req.json().await?;

    // Get user by email
    let db = get_d1(&ctx.env)?;
    if let Ok(Some(user)) = get_user_by_email(&db, &reset_data.email).await {
        // Generate reset token
        let token = password::generate_secure_token()
            .map_err(|e| worker::Error::RustError(format!("Failed to generate token: {}", e)))?;

        // Hash token for storage (deterministic hash for lookup)
        let token_hash = password::hash_token_for_lookup(&token);

        // Set expiry to 1 hour from now
        // Note: SQLite datetime format
        let expires_at = "datetime('now', '+1 hour')".to_string();

        create_password_reset_token(&db, &user.id, &token_hash, &expires_at)
            .await
            .map_err(|e| {
                worker::Error::RustError(format!("Failed to create reset token: {}", e))
            })?;

        // TODO: Send email with reset link
        // For now, just return success (prevent email enumeration)
    }

    // Always return 200 to prevent email enumeration
    Response::ok("Password reset email sent if account exists")
}

/// Confirm password reset
pub async fn confirm_password_reset(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    #[derive(serde::Deserialize)]
    struct ConfirmResetRequest {
        token: String,
        new_password: String,
    }

    let reset_data: ConfirmResetRequest = req.json().await?;

    // Validate password strength
    if reset_data.new_password.len() < 8 {
        return Response::error("Password must be at least 8 characters", 400);
    }

    // Hash token to look up (deterministic hash)
    let token_hash = password::hash_token_for_lookup(&reset_data.token);

    // Validate token
    let db = get_d1(&ctx.env)?;
    let user_id = validate_password_reset_token(&db, &token_hash)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to validate token: {}", e)))?;

    let user_id =
        user_id.ok_or_else(|| worker::Error::RustError("Invalid or expired token".to_string()))?;

    // Hash new password
    let password_hash = password::hash_password(&reset_data.new_password)
        .map_err(|e| worker::Error::RustError(format!("Failed to hash password: {}", e)))?;

    // Update password
    update_password(&db, &user_id, &password_hash)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to update password: {}", e)))?;

    Response::ok("Password reset successful")
}
