use anyhow::{anyhow, Result};
use serde_json::Value;
use worker::*;

use crate::services::password;
use crate::types::User;

use super::providers::link_provider;

/// Find or create user with account linking support
/// If provider_id exists, returns existing user_id
/// If email exists but provider_id doesn't, links provider to existing user (account linking)
/// Otherwise creates new user
pub async fn find_or_create_user(
    db: &D1Database,
    provider: &str,
    provider_id: &str,
    email: Option<&str>,
    name: Option<&str>,
    picture: Option<&str>,
) -> Result<String> {
    // First, check if this provider_id already exists
    let existing_provider = db
        .prepare("SELECT user_id FROM user_providers WHERE provider = ? AND provider_id = ?")
        .bind(&[provider.into(), provider_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = existing_provider {
        // Provider already linked, update user info and return user_id
        let user_id = row
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Failed to get user_id from provider"))?
            .to_string();

        // Update user info
        if let Some(email) = email {
            db.prepare("UPDATE users SET email = ?, name = ?, picture = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(&[
                    email.into(),
                    name.unwrap_or("").into(),
                    picture.unwrap_or("").into(),
                    user_id.clone().into(),
                ])?
                .run()
                .await?;
        }

        return Ok(user_id);
    }

    // Provider doesn't exist, check if email exists (account linking)
    if let Some(email) = email {
        let existing_user = db
            .prepare("SELECT id FROM users WHERE email = ?")
            .bind(&[email.into()])?
            .first::<Value>(None)
            .await?;

        if let Some(row) = existing_user {
            // Email exists, link provider to existing user (account linking)
            let user_id = row
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Failed to get user_id from email"))?
                .to_string();

            // Update user info
            db.prepare("UPDATE users SET name = ?, picture = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(&[
                    name.unwrap_or("").into(),
                    picture.unwrap_or("").into(),
                    user_id.clone().into(),
                ])?
                .run()
                .await?;

            // Link provider
            link_provider(db, &user_id, provider, provider_id).await?;

            return Ok(user_id);
        }
    }

    // Create new user with UUID
    let user_id =
        password::generate_uuid().map_err(|e| anyhow!("Failed to generate UUID: {}", e))?;

    db.prepare("INSERT INTO users (id, email, name, picture) VALUES (?, ?, ?, ?)")
        .bind(&[
            user_id.clone().into(),
            email.unwrap_or("").into(),
            name.unwrap_or("").into(),
            picture.unwrap_or("").into(),
        ])?
        .run()
        .await?;

    // Link provider
    link_provider(db, &user_id, provider, provider_id).await?;

    Ok(user_id)
}

/// Get user by ID with providers list
pub async fn get_user_by_id(db: &D1Database, user_id: &str) -> Result<Option<User>> {
    let result = db
        .prepare("SELECT * FROM users WHERE id = ?")
        .bind(&[user_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        // Get providers list
        let providers_result = db
            .prepare("SELECT provider FROM user_providers WHERE user_id = ?")
            .bind(&[user_id.into()])?
            .all()
            .await?;

        let providers: Vec<String> = providers_result
            .results()?
            .iter()
            .filter_map(|p: &Value| {
                p.get("provider")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect();

        let user = User {
            id: row
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing user id"))?
                .to_string(),
            email: row
                .get("email")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            picture: row
                .get("picture")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: row
                .get("created_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            updated_at: row
                .get("updated_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            providers,
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

/// Get user by email
pub async fn get_user_by_email(db: &D1Database, email: &str) -> Result<Option<User>> {
    let result = db
        .prepare("SELECT * FROM users WHERE email = ?")
        .bind(&[email.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        let user_id = row
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing user id"))?
            .to_string();

        // Get providers list
        let providers_result = db
            .prepare("SELECT provider FROM user_providers WHERE user_id = ?")
            .bind(&[user_id.clone().into()])?
            .all()
            .await?;

        let providers: Vec<String> = providers_result
            .results()?
            .iter()
            .filter_map(|p: &Value| {
                p.get("provider")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect();

        let user = User {
            id: user_id,
            email: row
                .get("email")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            picture: row
                .get("picture")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: row
                .get("created_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            updated_at: row
                .get("updated_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            providers,
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

/// Create local user with password
pub async fn create_local_user(
    db: &D1Database,
    email: &str,
    name: &str,
    password_hash: &str,
) -> Result<String> {
    // Check if email already exists
    let existing = db
        .prepare("SELECT id FROM users WHERE email = ?")
        .bind(&[email.into()])?
        .first::<Value>(None)
        .await?;

    if existing.is_some() {
        return Err(anyhow!("Email already registered"));
    }

    // Generate UUID for new user
    let user_id =
        password::generate_uuid().map_err(|e| anyhow!("Failed to generate UUID: {}", e))?;

    // Create user
    db.prepare("INSERT INTO users (id, email, name) VALUES (?, ?, ?)")
        .bind(&[user_id.clone().into(), email.into(), name.into()])?
        .run()
        .await?;

    // Link local provider
    db.prepare("INSERT INTO user_providers (user_id, provider, provider_id) VALUES (?, ?, ?)")
        .bind(&[user_id.clone().into(), "local".into(), email.into()])?
        .run()
        .await?;

    // Create credentials
    db.prepare("INSERT INTO user_credentials (user_id, password_hash) VALUES (?, ?)")
        .bind(&[user_id.clone().into(), password_hash.into()])?
        .run()
        .await?;

    Ok(user_id)
}
