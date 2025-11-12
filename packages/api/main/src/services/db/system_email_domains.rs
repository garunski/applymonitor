use anyhow::Result;
use worker::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SystemEmailDomain {
    pub id: String,
    pub domain_pattern: String,
    pub name: Option<String>,
}

/// Get all system email domain patterns
pub async fn get_all_domains(db: &D1Database) -> Result<Vec<SystemEmailDomain>> {
    let result = db
        .prepare(
            "SELECT id, domain_pattern, name FROM system_email_domains ORDER BY domain_pattern",
        )
        .all()
        .await?;

    let rows: Vec<serde_json::Value> = result.results()?;
    let mut domains: Vec<SystemEmailDomain> = Vec::new();

    for row in rows {
        domains.push(SystemEmailDomain {
            id: row
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            domain_pattern: row
                .get("domain_pattern")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        });
    }

    Ok(domains)
}

/// Check if an email address matches any system email domain pattern
pub async fn is_system_email(db: &D1Database, email: &str) -> Result<bool> {
    // URL decode the email if needed (handles %40 -> @)
    // Simple approach: replace common URL-encoded characters
    let decoded_email = email.replace("%40", "@").replace("%2E", ".");

    // Extract local part (before @) and domain (after @)
    let (local_part, domain) = if let Some(at_pos) = decoded_email.find('@') {
        (&decoded_email[..at_pos], &decoded_email[at_pos + 1..])
    } else {
        return Ok(false);
    };

    let domains = get_all_domains(db).await?;

    for system_domain in domains {
        // Check if pattern matches domain (e.g., "*.greenhouse.io")
        if matches_pattern_impl(&system_domain.domain_pattern, domain) {
            return Ok(true);
        }
        // Check if pattern matches local part (e.g., "noreply.*" matches "noreply@anything.com")
        if matches_pattern_impl(&system_domain.domain_pattern, local_part) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check if a domain matches a pattern (supports wildcard *)
/// Exposed for testing
pub fn matches_pattern(pattern: &str, domain: &str) -> bool {
    matches_pattern_impl(pattern, domain)
}

/// Internal implementation of pattern matching
fn matches_pattern_impl(pattern: &str, domain: &str) -> bool {
    if pattern == domain {
        return true;
    }

    // Support wildcard patterns like "noreply.*" or "*.greenhouse.io"
    if pattern.contains('*') {
        let pattern_parts: Vec<&str> = pattern.split('*').collect();

        if pattern_parts.len() == 2 {
            let prefix = pattern_parts[0];
            let suffix = pattern_parts[1];

            if prefix.is_empty() {
                // Pattern: "*suffix" (e.g., "*.greenhouse.io")
                // Match if domain ends with suffix OR if domain equals suffix without leading dot
                if suffix.starts_with('.') {
                    let suffix_without_dot = &suffix[1..];
                    return domain.ends_with(suffix) || domain == suffix_without_dot;
                }
                return domain.ends_with(suffix);
            } else if suffix.is_empty() {
                // Pattern: "prefix*" (e.g., "noreply.*" splits to ["noreply.", ""])
                // Remove trailing dot from prefix if present, then check if it starts with prefix
                let prefix_clean = prefix.strip_suffix('.').unwrap_or(prefix);
                return domain.starts_with(prefix_clean);
            } else if suffix == "." {
                // Pattern: "prefix.*" means "prefix" followed by anything (including nothing)
                // Remove trailing dot from prefix if present, then check if it starts with prefix
                let prefix_clean = prefix.strip_suffix('.').unwrap_or(prefix);
                return domain.starts_with(prefix_clean);
            } else {
                // Pattern: "prefix*suffix"
                return domain.starts_with(prefix) && domain.ends_with(suffix);
            }
        }
    }

    false
}
