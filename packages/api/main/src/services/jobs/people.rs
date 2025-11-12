//! People extraction from emails

use serde_json::Value;
use std::collections::HashSet;

/// Extract unique people from email "from" fields
/// Parses "Name <email@example.com>" format
pub fn extract_people_from_emails(emails: &[Value]) -> Vec<Value> {
    let mut people: Vec<Value> = Vec::new();
    let mut seen_emails: HashSet<String> = HashSet::new();

    for email in emails {
        if let Some(from) = email.get("from").and_then(|v| v.as_str()) {
            // Extract email address from "Name <email@example.com>" format
            let email_addr = if let Some(start) = from.find('<') {
                if let Some(end) = from.find('>') {
                    &from[start + 1..end]
                } else {
                    from
                }
            } else {
                from
            };

            if !seen_emails.contains(email_addr) {
                seen_emails.insert(email_addr.to_string());
                people.push(serde_json::json!({
                    "email": email_addr,
                    "name": from.split('<').next().unwrap_or(from).trim(),
                }));
            }
        }
    }

    people
}
