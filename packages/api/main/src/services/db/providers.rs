use crate::services::password;
use anyhow::{anyhow, Result};
use serde_json::Value;
use worker::*;

/// Link a provider to an existing user
pub async fn link_provider(
    db: &D1Database,
    user_id: &str,
    provider: &str,
    provider_id: &str,
) -> Result<()> {
    // Check if already linked
    let existing = db
        .prepare(
            "SELECT id FROM user_providers WHERE user_id = ? AND provider = ? AND provider_id = ?",
        )
        .bind(&[user_id.into(), provider.into(), provider_id.into()])?
        .first::<Value>(None)
        .await?;

    if existing.is_some() {
        // Already linked
        return Ok(());
    }

    // Check if provider_id is already linked to a different user
    let conflict = db
        .prepare("SELECT user_id FROM user_providers WHERE provider = ? AND provider_id = ?")
        .bind(&[provider.into(), provider_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = conflict {
        let linked_user_id = row
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Failed to get user_id"))?
            .to_string();

        if linked_user_id != user_id {
            return Err(anyhow!("Provider already linked to another account"));
        }
    }

    // Link provider
    let provider_uuid =
        password::generate_uuid().map_err(|e| anyhow!("Failed to generate UUID: {}", e))?;
    db.prepare(
        "INSERT INTO user_providers (id, user_id, provider, provider_id) VALUES (?, ?, ?, ?)",
    )
    .bind(&[
        provider_uuid.into(),
        user_id.into(),
        provider.into(),
        provider_id.into(),
    ])?
    .run()
    .await?;

    Ok(())
}

/// Unlink provider from user
pub async fn unlink_provider(db: &D1Database, user_id: &str, provider: &str) -> Result<()> {
    // Check how many providers user has
    let providers_result = db
        .prepare("SELECT COUNT(*) as count FROM user_providers WHERE user_id = ?")
        .bind(&[user_id.into()])?
        .first::<Value>(None)
        .await?;

    let count = providers_result
        .and_then(|row| row.get("count").and_then(|v| v.as_i64()))
        .unwrap_or(0i64);

    if count <= 1 {
        return Err(anyhow!("Cannot unlink last provider"));
    }

    // Unlink provider
    db.prepare("DELETE FROM user_providers WHERE user_id = ? AND provider = ?")
        .bind(&[user_id.into(), provider.into()])?
        .run()
        .await?;

    // If unlinking local provider, delete credentials
    if provider == "local" {
        db.prepare("DELETE FROM user_credentials WHERE user_id = ?")
            .bind(&[user_id.into()])?
            .run()
            .await?;
    }

    Ok(())
}
