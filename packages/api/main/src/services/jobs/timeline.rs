//! Timeline event building for jobs

use serde_json::Value;

/// Build timeline events from job, emails, and comments
pub fn build_timeline_events(job: &Value, emails: &[Value], comments: &[Value]) -> Vec<Value> {
    let mut timeline_events: Vec<Value> = Vec::new();

    // Job creation event
    if let Some(created_at) = job.get("created_at").and_then(|v| v.as_str()) {
        timeline_events.push(serde_json::json!({
            "type": "job_created",
            "timestamp": created_at,
            "data": {
                "title": job.get("title").and_then(|v| v.as_str()).unwrap_or(""),
            }
        }));
    }

    // Status change events (if updated_at differs from created_at, assume status changed)
    if let (Some(created_at), Some(updated_at)) = (
        job.get("created_at").and_then(|v| v.as_str()),
        job.get("updated_at").and_then(|v| v.as_str()),
    ) {
        if created_at != updated_at {
            timeline_events.push(serde_json::json!({
                "type": "status_changed",
                "timestamp": updated_at,
                "data": {
                    "status": job.get("status").and_then(|v| v.as_str()).unwrap_or(""),
                }
            }));
        }
    }

    // Email events
    for email in emails {
        if let Some(date) = email.get("date").and_then(|v| v.as_str()) {
            timeline_events.push(serde_json::json!({
                "type": "email_received",
                "timestamp": date,
                "data": {
                    "subject": email.get("subject").and_then(|v| v.as_str()).unwrap_or(""),
                    "from": email.get("from").and_then(|v| v.as_str()).unwrap_or(""),
                    "gmail_id": email.get("gmail_id").and_then(|v| v.as_str()).unwrap_or(""),
                }
            }));
        }
    }

    // Comment events
    for comment in comments {
        if let Some(created_at) = comment.get("created_at").and_then(|v| v.as_str()) {
            timeline_events.push(serde_json::json!({
                "type": "comment_added",
                "timestamp": created_at,
                "data": {
                    "content": comment.get("content").and_then(|v| v.as_str()).unwrap_or(""),
                    "user_name": comment.get("name").and_then(|v| v.as_str()),
                    "user_picture": comment.get("picture").and_then(|v| v.as_str()),
                }
            }));
        }
    }

    // Sort timeline events by timestamp (newest first)
    timeline_events.sort_by(|a, b| {
        let a_ts = a.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        let b_ts = b.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        b_ts.cmp(a_ts)
    });

    timeline_events
}
