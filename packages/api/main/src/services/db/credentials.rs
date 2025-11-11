use crate::services::password;
use anyhow::{anyhow, Result};
use serde_json::Value;
use worker::*;

/// Get password hash for user
pub async fn get_password_hash(db: &D1Database, user_id: &str) -> Result<Option<String>> {
    let result = db
        .prepare("SELECT password_hash FROM user_credentials WHERE user_id = ?")
        .bind(&[user_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        Ok(row
            .get("password_hash")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()))
    } else {
        Ok(None)
    }
}

/// Update password hash
pub async fn update_password(db: &D1Database, user_id: &str, password_hash: &str) -> Result<()> {
    db.prepare("UPDATE user_credentials SET password_hash = ?, updated_at = CURRENT_TIMESTAMP WHERE user_id = ?")
        .bind(&[password_hash.into(), user_id.into()])?
        .run()
        .await?;
    Ok(())
}

/// Create password reset token
pub async fn create_password_reset_token(
    db: &D1Database,
    user_id: &str,
    token_hash: &str,
    expires_at: &str,
) -> Result<()> {
    let token_uuid =
        password::generate_uuid().map_err(|e| anyhow!("Failed to generate UUID: {}", e))?;
    db.prepare(
        "INSERT INTO password_reset_tokens (id, user_id, token_hash, expires_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&[token_uuid.into(), user_id.into(), token_hash.into(), expires_at.into()])?
    .run()
    .await?;
    Ok(())
}

/// Validate password reset token
pub async fn validate_password_reset_token(
    db: &D1Database,
    token_hash: &str,
) -> Result<Option<String>> {
    // Check if token exists and is not expired
    let result = db
        .prepare("SELECT user_id, expires_at FROM password_reset_tokens WHERE token_hash = ? AND expires_at > datetime('now')")
        .bind(&[token_hash.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        let user_id = row
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing user_id"))?
            .to_string();

        // Delete token after validation (one-time use)
        db.prepare("DELETE FROM password_reset_tokens WHERE token_hash = ?")
            .bind(&[token_hash.into()])?
            .run()
            .await?;

        Ok(Some(user_id))
    } else {
        Ok(None)
    }
}
