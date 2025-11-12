use anyhow::{anyhow, Result};
use serde_json::Value;
use worker::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailContact {
    pub email: String,
    pub user_id: String,
    pub name: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
    pub is_system: bool,
}

/// Normalize email address (lowercase, trim)
fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Extract email address from "Name <email@example.com>" format
fn extract_email_address(email_string: &str) -> String {
    let email = if let Some(start) = email_string.find('<') {
        if let Some(end) = email_string.find('>') {
            normalize_email(&email_string[start + 1..end])
        } else {
            normalize_email(email_string)
        }
    } else {
        normalize_email(email_string)
    };
    // Only return if it's a valid email (contains @)
    if email.contains('@') {
        email
    } else {
        String::new()
    }
}

/// Get or create a contact for an email address
pub async fn get_or_create_contact(
    db: &D1Database,
    email: &str,
    user_id: &str,
) -> Result<EmailContact> {
    let normalized_email = extract_email_address(email);

    // Ensure we have a valid email
    if normalized_email.is_empty() || !normalized_email.contains('@') {
        return Err(anyhow::anyhow!("Invalid email address: {}", email));
    }

    // Try to get existing contact
    let result = db
        .prepare("SELECT * FROM email_contacts WHERE email = ? AND user_id = ?")
        .bind(&[normalized_email.clone().into(), user_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        return Ok(EmailContact {
            email: normalized_email,
            user_id: user_id.to_string(),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            linkedin: row
                .get("linkedin")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            website: row
                .get("website")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            is_system: row
                .get("is_system")
                .and_then(|v| v.as_i64())
                .map(|i| i != 0)
                .unwrap_or(false),
        });
    }

    // Extract name from email string if available
    let name = if email.contains('<') {
        email
            .split('<')
            .next()
            .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\'').to_string())
    } else {
        None
    };

    // Create new contact - handle optional name field by duplicating INSERT
    match &name {
        Some(name_str) => {
            db.prepare(
                "INSERT INTO email_contacts (email, user_id, name, is_system, created_at, updated_at) VALUES (?, ?, ?, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            )
            .bind(&[
                normalized_email.clone().into(),
                user_id.into(),
                name_str.as_str().into(),
            ])?
            .run()
            .await?;
        }
        None => {
            db.prepare(
                "INSERT INTO email_contacts (email, user_id, name, is_system, created_at, updated_at) VALUES (?, ?, NULL, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            )
            .bind(&[
                normalized_email.clone().into(),
                user_id.into(),
            ])?
            .run()
            .await?;
        }
    }

    Ok(EmailContact {
        email: normalized_email,
        user_id: user_id.to_string(),
        name,
        linkedin: None,
        website: None,
        is_system: false,
    })
}

/// Get a specific contact
pub async fn get_contact(
    db: &D1Database,
    email: &str,
    user_id: &str,
) -> Result<Option<EmailContact>> {
    let normalized_email = normalize_email(email);

    let result = db
        .prepare("SELECT * FROM email_contacts WHERE email = ? AND user_id = ?")
        .bind(&[normalized_email.clone().into(), user_id.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        Ok(Some(EmailContact {
            email: normalized_email,
            user_id: user_id.to_string(),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            linkedin: row
                .get("linkedin")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            website: row
                .get("website")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            is_system: row
                .get("is_system")
                .and_then(|v| v.as_i64())
                .map(|i| i != 0)
                .unwrap_or(false),
        }))
    } else {
        Ok(None)
    }
}

/// Update a contact
pub async fn update_contact(
    db: &D1Database,
    email: &str,
    user_id: &str,
    name: Option<&str>,
    linkedin: Option<&str>,
    website: Option<&str>,
) -> Result<EmailContact> {
    let normalized_email = normalize_email(email);

    // Ensure contact exists first
    get_or_create_contact(db, email, user_id).await?;

    // Update contact
    db.prepare(
        "UPDATE email_contacts SET name = ?, linkedin = ?, website = ?, updated_at = CURRENT_TIMESTAMP WHERE email = ? AND user_id = ?",
    )
    .bind(&[
        name.into(),
        linkedin.into(),
        website.into(),
        normalized_email.clone().into(),
        user_id.into(),
    ])?
    .run()
    .await?;

    // Return updated contact
    get_contact(db, &normalized_email, user_id)
        .await?
        .ok_or_else(|| anyhow!("Contact not found after update"))
}

/// Get all contacts for a job (from emails linked to the job)
pub async fn get_contacts_for_job(
    db: &D1Database,
    job_id: &str,
    user_id: &str,
) -> Result<Vec<EmailContact>> {
    // Get all unique emails from emails table for this job (excluding "to" field)
    let result = db
        .prepare(
            "SELECT DISTINCT email FROM (
                SELECT \"from\" as email FROM emails WHERE job_id = ? AND user_id = ? AND \"from\" IS NOT NULL
                UNION
                SELECT cc as email FROM emails WHERE job_id = ? AND user_id = ? AND cc IS NOT NULL
                UNION
                SELECT bcc as email FROM emails WHERE job_id = ? AND user_id = ? AND bcc IS NOT NULL
            )",
        )
        .bind(&[
            job_id.into(),
            user_id.into(),
            job_id.into(),
            user_id.into(),
            job_id.into(),
            user_id.into(),
        ])?
        .all()
        .await?;

    let rows: Vec<serde_json::Value> = result.results()?;
    let mut contacts: Vec<EmailContact> = Vec::new();

    for row in rows {
        if let Some(email_str) = row.get("email").and_then(|v| v.as_str()) {
            // Extract individual email addresses from the field (may contain multiple)
            let emails = extract_emails_from_field(email_str);
            for email in emails {
                if let Ok(contact) = get_or_create_contact(db, &email, user_id).await {
                    // Avoid duplicates
                    if !contacts
                        .iter()
                        .any(|c: &EmailContact| c.email == contact.email)
                    {
                        contacts.push(contact);
                    }
                }
            }
        }
    }

    Ok(contacts)
}

/// Extract individual email addresses from a field that may contain multiple emails
fn extract_emails_from_field(field_value: &str) -> Vec<String> {
    let mut emails = Vec::new();

    // Split by comma and process each part
    for part in field_value.split(',') {
        let trimmed = part.trim();
        if !trimmed.is_empty() {
            let email = extract_email_address(trimmed);
            if !email.is_empty() {
                emails.push(email);
            }
        }
    }

    emails
}
