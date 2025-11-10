use anyhow::{anyhow, Result};
use serde_json::Value;
use worker::*;

use crate::types::User;

pub async fn upsert_user(
    db: &D1Database,
    provider: &str,
    provider_sub: &str,
    email: Option<&str>,
    name: Option<&str>,
    avatar: Option<&str>,
) -> Result<i64> {
    // Check if user exists
    let existing = db
        .prepare("SELECT id FROM users WHERE provider = ? AND provider_sub = ?")
        .bind(&[provider.into(), provider_sub.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = existing {
        // Update existing user
        let id = row
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow!("Failed to get user ID"))?;

        db.prepare(
            "UPDATE users SET email = ?, name = ?, avatar = ?, last_login = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(&[
            email.unwrap_or("").into(),
            name.unwrap_or("").into(),
            avatar.unwrap_or("").into(),
            id.into(),
        ])?
        .run()
        .await?;

        Ok(id)
    } else {
        // Insert new user
        db.prepare(
            "INSERT INTO users (provider, provider_sub, email, name, avatar, last_login) VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)"
        )
        .bind(&[
            provider.into(),
            provider_sub.into(),
            email.unwrap_or("").into(),
            name.unwrap_or("").into(),
            avatar.unwrap_or("").into(),
        ])?
        .run()
        .await?;

        let result = db
            .prepare("SELECT last_insert_rowid() as id")
            .first::<Value>(None)
            .await?;

        let id = result
            .ok_or_else(|| anyhow!("Failed to get inserted user ID"))?
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow!("Failed to parse user ID"))?;

        Ok(id)
    }
}

pub async fn get_user_by_id(db: &D1Database, user_id: i64) -> Result<Option<User>> {
    let result = db
        .prepare("SELECT * FROM users WHERE id = ?")
        .bind(&[user_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        let user = User {
            id: row
                .get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing user id"))?,
            provider: row
                .get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            provider_sub: row
                .get("provider_sub")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            email: row
                .get("email")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            avatar: row
                .get("avatar")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: row
                .get("created_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            last_login: row
                .get("last_login")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
