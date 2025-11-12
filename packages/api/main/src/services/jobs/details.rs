//! Job details data fetching

use crate::common::auth::require_auth;
use crate::services::jobs::{
    build_timeline_events, extract_people_from_emails, normalize_job_id, process_contacts_for_job,
};
use serde_json::Value;
use worker::{D1Database, Env, Request};

/// Job details data structure
pub struct JobDetailsData {
    pub job: Value,
    pub emails: Vec<Value>,
    pub comments: Vec<Value>,
    pub timeline_events: Vec<Value>,
    pub people: Vec<Value>,
    pub contacts: Vec<Value>,
}

/// Get job details data including emails, comments, timeline, people, and contacts
pub async fn get_job_details_data(
    db: &D1Database,
    req: &Request,
    env: &Env,
    id: String,
) -> Result<JobDetailsData, worker::Error> {
    let user_id = require_auth(req, env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    // Get job
    let job_result = db
        .prepare(
            "SELECT 
                j.*, 
                j.status_id,
                js.name as status_name
            FROM jobs j
            LEFT JOIN job_statuses js ON j.status_id = js.id
            WHERE j.id = ?",
        )
        .bind(&[id.clone().into()])?
        .first::<Value>(None)
        .await?;

    let mut job = match job_result {
        Some(j) => j,
        None => return Err(worker::Error::RustError("Job not found".to_string())),
    };
    normalize_job_id(&mut job);

    // Get emails linked to this job
    let emails_result = db
        .prepare("SELECT * FROM emails WHERE job_id = ? AND user_id = ? ORDER BY date DESC")
        .bind(&[id.clone().into(), user_id.clone().into()])?
        .all()
        .await?;

    let emails: Vec<Value> = emails_result.results()?;

    // Get comments for this job
    let comments_result = db
        .prepare(
            "SELECT jc.*, u.name, u.email, u.picture FROM job_comments jc 
             LEFT JOIN users u ON jc.user_id = u.id 
             WHERE jc.job_id = ? ORDER BY jc.created_at DESC",
        )
        .bind(&[id.clone().into()])?
        .all()
        .await?;

    let comments: Vec<Value> = comments_result.results()?;

    // Build timeline events
    let timeline_events = build_timeline_events(&job, &emails, &comments);

    // Extract unique people from emails (legacy, keep for backward compatibility)
    let people = extract_people_from_emails(&emails);

    // Process contacts with system email detection
    let contacts = process_contacts_for_job(db, &id, &user_id).await?;

    Ok(JobDetailsData {
        job,
        emails,
        comments,
        timeline_events,
        people,
        contacts,
    })
}
