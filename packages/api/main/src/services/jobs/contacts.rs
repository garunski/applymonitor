//! Contact processing for jobs

use crate::services::db::email_contacts::get_contacts_for_job;
use crate::services::db::system_email_domains::is_system_email;
use serde_json::Value;
use worker::D1Database;

/// Process contacts for a job, checking system email detection
pub async fn process_contacts_for_job(
    db: &D1Database,
    job_id: &str,
    user_id: &str,
) -> Result<Vec<Value>, worker::Error> {
    let contacts = get_contacts_for_job(db, job_id, user_id)
        .await
        .unwrap_or_default();

    let mut contacts_json: Vec<Value> = Vec::new();
    for contact in contacts {
        // Only run system email detection if contact is not already saved as system contact
        // If already a system contact, skip detection (just show badge)
        let is_system_detected = if contact.is_system {
            // Already a system contact, no need to detect
            false
        } else {
            // Check if it's a system email (for conversion suggestion)
            is_system_email(db, &contact.email).await.unwrap_or(false)
        };

        let contact_json = serde_json::json!({
            "email": contact.email,
            "user_id": contact.user_id,
            "name": contact.name,
            "linkedin": contact.linkedin,
            "website": contact.website,
            // Only show badge if contact was saved/converted as system contact, not just detected
            "is_system": contact.is_system,
            "is_system_detected": is_system_detected,
        });
        contacts_json.push(contact_json);
    }

    Ok(contacts_json)
}
