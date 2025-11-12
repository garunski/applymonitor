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
    // Extract domain from email
    let domain = if let Some(at_pos) = email.find('@') {
        &email[at_pos + 1..]
    } else {
        return Ok(false);
    };

    let domains = get_all_domains(db).await?;

    for system_domain in domains {
        if matches_pattern(&system_domain.domain_pattern, domain) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check if a domain matches a pattern (supports wildcard *)
fn matches_pattern(pattern: &str, domain: &str) -> bool {
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
                // Pattern: "*suffix"
                return domain.ends_with(suffix);
            } else if suffix.is_empty() {
                // Pattern: "prefix*"
                return domain.starts_with(prefix);
            } else {
                // Pattern: "prefix*suffix"
                return domain.starts_with(prefix) && domain.ends_with(suffix);
            }
        }
    }

    false
}
