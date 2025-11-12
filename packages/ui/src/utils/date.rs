//! Date formatting utilities with timezone support

use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::Tz;
use std::str::FromStr;

/// Parse a date string in RFC3339 or email format and convert to UTC
fn parse_to_utc(date_str: &str) -> Option<DateTime<Utc>> {
    // Try RFC3339 format first
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try email format: "Mon, 15 Jan 2024 14:30:00 +0000"
    if let Ok(dt) = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z") {
        return Some(dt.with_timezone(&Utc));
    }

    None
}

/// Convert UTC datetime to user's timezone
fn to_user_timezone(dt: DateTime<Utc>, timezone: Option<&str>) -> DateTime<FixedOffset> {
    match timezone {
        Some(tz_str) => {
            if let Ok(tz) = Tz::from_str(tz_str) {
                // Convert UTC to timezone
                let dt_in_tz = dt.with_timezone(&tz);
                // Calculate offset by difference between local and UTC naive times
                let offset_secs = (dt_in_tz.naive_local() - dt_in_tz.naive_utc()).num_seconds();
                // Create FixedOffset (positive means ahead of UTC)
                let offset = FixedOffset::east_opt(offset_secs as i32)
                    .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
                dt_in_tz.with_timezone(&offset)
            } else {
                // Invalid timezone, fallback to UTC
                dt.with_timezone(&FixedOffset::east_opt(0).unwrap())
            }
        }
        None => {
            // No timezone set, use UTC
            dt.with_timezone(&FixedOffset::east_opt(0).unwrap())
        }
    }
}

/// Format date as "Jan 15, 2024"
pub fn format_date(date_str: &str, timezone: Option<&str>) -> String {
    if let Some(dt_utc) = parse_to_utc(date_str) {
        let dt = to_user_timezone(dt_utc, timezone);
        dt.format("%b %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

/// Format date as "January 15, 2024 at 3:45 PM"
pub fn format_date_full(date_str: &str, timezone: Option<&str>) -> String {
    if let Some(dt_utc) = parse_to_utc(date_str) {
        let dt = to_user_timezone(dt_utc, timezone);
        dt.format("%B %d, %Y at %I:%M %p").to_string()
    } else {
        date_str.to_string()
    }
}

/// Format relative time as "2h ago", "3d ago", etc.
/// Note: Relative time is calculated in UTC regardless of timezone
pub fn format_relative_time(timestamp: &str, _timezone: Option<&str>) -> String {
    if let Some(dt_utc) = parse_to_utc(timestamp) {
        let now = Utc::now();
        let duration = now.signed_duration_since(dt_utc);

        if duration.num_days() > 0 {
            format!("{}d ago", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{}h ago", duration.num_hours())
        } else if duration.num_minutes() > 0 {
            format!("{}m ago", duration.num_minutes())
        } else {
            "just now".to_string()
        }
    } else {
        timestamp.to_string()
    }
}
