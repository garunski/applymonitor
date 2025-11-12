//! Email parsing utilities

/// Parse email address from "Name <email@example.com>" format
/// Returns (name, email) tuple
pub fn parse_email_address(email_string: &str) -> (Option<String>, String) {
    let trimmed = email_string.trim();

    if let Some(start) = trimmed.find('<') {
        if let Some(end) = trimmed.find('>') {
            let email = trimmed[start + 1..end].trim().to_lowercase();
            let name = trimmed[..start].trim();
            let name_clean = name.trim_matches(|c| c == '"' || c == '\'').trim();
            let name_opt = if name_clean.is_empty() {
                None
            } else {
                Some(name_clean.to_string())
            };
            return (name_opt, email);
        }
    }

    // No angle brackets, treat entire string as email
    (None, trimmed.to_lowercase())
}

/// Extract individual email addresses from a field that may contain multiple emails
/// Handles comma-separated lists and "Name <email>" format
pub fn extract_emails_from_field(field_value: &str) -> Vec<String> {
    let mut emails = Vec::new();

    // Split by comma and process each part
    for part in field_value.split(',') {
        let trimmed = part.trim();
        if !trimmed.is_empty() {
            let (_, email) = parse_email_address(trimmed);
            if !email.is_empty() {
                emails.push(email);
            }
        }
    }

    emails
}

/// Normalize email address (lowercase, trim)
pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}
