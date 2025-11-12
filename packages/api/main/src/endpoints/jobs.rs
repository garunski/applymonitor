use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::password;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use worker::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Option<String>,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

fn normalize_job_id(job: &mut serde_json::Value) {
    if let Some(id_val) = job.get_mut("id") {
        if let Some(id_str) = id_val.as_str() {
            *id_val = serde_json::Value::String(id_str.to_string());
        } else if let Some(id_int) = id_val.as_i64() {
            *id_val = serde_json::Value::String(id_int.to_string());
        } else if let Some(id_int) = id_val.as_u64() {
            *id_val = serde_json::Value::String(id_int.to_string());
        }
    }
}

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let method = req.method();

    let job_id = ctx.param("id").map(|s| s.to_string());
    let url = req.url()?;
    let query_params = url
        .query_pairs()
        .collect::<std::collections::HashMap<_, _>>();
    let include_details = query_params
        .get("include")
        .map(|s| s == "details")
        .unwrap_or(false);

    match method {
        Method::Get => {
            if let Some(id) = job_id {
                if include_details {
                    get_job_details(&db, &req, &ctx.env, id).await
                } else {
                    get_job(&db, id).await
                }
            } else {
                list_jobs(&db).await
            }
        }
        Method::Post => create_job(&db, req).await,
        Method::Put => {
            if let Some(id) = job_id {
                update_job(&db, req, &ctx.env, id).await
            } else {
                Response::error("Job ID is required for update", 400)
            }
        }
        Method::Delete => {
            if let Some(id) = job_id {
                delete_job(&db, id).await
            } else {
                Response::error("Job ID is required for deletion", 400)
            }
        }
        _ => Response::error("Method not allowed", 405),
    }
}

async fn list_jobs(db: &D1Database) -> Result<Response> {
    let result = db
        .prepare("SELECT * FROM jobs ORDER BY created_at DESC")
        .all()
        .await?;

    let mut jobs: Vec<serde_json::Value> = result.results()?;
    for job in &mut jobs {
        normalize_job_id(job);
    }
    Response::from_json(&jobs)
}

async fn get_job(db: &D1Database, id: String) -> Result<Response> {
    let result = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    match result {
        Some(mut job) => {
            normalize_job_id(&mut job);
            Response::from_json(&job)
        }
        None => Response::error("Job not found", 404),
    }
}

async fn get_job_details(
    db: &D1Database,
    req: &Request,
    env: &Env,
    id: String,
) -> Result<Response> {
    let user_id = require_auth(req, env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    // Get job
    let job_result = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[id.clone().into()])?
        .first::<Value>(None)
        .await?;

    let job = match job_result {
        Some(mut j) => {
            normalize_job_id(&mut j);
            j
        }
        None => return Response::error("Job not found", 404),
    };

    // Get emails linked to this job
    let emails_result = db
        .prepare("SELECT * FROM emails WHERE job_id = ? AND user_id = ? ORDER BY date DESC")
        .bind(&[id.clone().into(), user_id.into()])?
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
    for email in &emails {
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
    for comment in &comments {
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

    // Extract unique people from emails
    let mut people: Vec<Value> = Vec::new();
    let mut seen_emails: std::collections::HashSet<String> = std::collections::HashSet::new();

    for email in &emails {
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

    let response = serde_json::json!({
        "job": job,
        "emails": emails,
        "comments": comments,
        "timeline_events": timeline_events,
        "people": people,
    });

    Response::from_json(&response)
}

async fn create_job(db: &D1Database, mut req: Request) -> Result<Response> {
    let job: Job = req.json().await?;

    if job.title.is_empty() || job.company.is_empty() {
        return Response::error("Title and company are required", 400);
    }

    let job_id = password::generate_uuid()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate UUID: {}", e)))?;

    match (&job.location, &job.description) {
        (Some(location), Some(description)) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status, description) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                location.as_str().into(),
                job.status.clone().into(),
                description.as_str().into(),
            ])?
            .run()
            .await?;
        }
        (Some(location), None) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status, description) VALUES (?, ?, ?, ?, ?, NULL)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                location.as_str().into(),
                job.status.clone().into(),
            ])?
            .run()
            .await?;
        }
        (None, Some(description)) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status, description) VALUES (?, ?, ?, NULL, ?, ?)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                job.status.clone().into(),
                description.as_str().into(),
            ])?
            .run()
            .await?;
        }
        (None, None) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status, description) VALUES (?, ?, ?, NULL, ?, NULL)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                job.status.clone().into(),
            ])?
            .run()
            .await?;
        }
    }

    let created_job = serde_json::json!({
        "id": job_id,
        "title": job.title,
        "company": job.company,
        "location": job.location,
        "status": job.status,
        "description": job.description,
        "created_at": null,
        "updated_at": null
    });

    let resp = Response::from_json(&created_job)?;
    Ok(resp.with_status(201))
}

async fn update_job(db: &D1Database, mut req: Request, _env: &Env, id: String) -> Result<Response> {
    // Check if this is a description-only update (PATCH-like behavior via query param)
    let url = req.url()?;
    let query_params = url
        .query_pairs()
        .collect::<std::collections::HashMap<_, _>>();
    let description_only = query_params
        .get("description_only")
        .map(|s| s == "true")
        .unwrap_or(false);

    if description_only {
        // Handle description-only update
        let body: serde_json::Value = req.json().await?;
        let description = body.get("description").and_then(|v| v.as_str());

        if let Some(desc) = description {
            db.prepare(
                "UPDATE jobs SET description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            )
            .bind(&[desc.into(), id.clone().into()])?
            .run()
            .await?;
        } else {
            db.prepare(
                "UPDATE jobs SET description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            )
            .bind(&[id.clone().into()])?
            .run()
            .await?;
        }
    } else {
        // Full job update
        let job: Job = req.json().await?;

        if job.title.is_empty() || job.company.is_empty() {
            return Response::error("Title and company are required", 400);
        }

        match (&job.location, &job.description) {
            (Some(location), Some(description)) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = ?, status = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    location.as_str().into(),
                    job.status.into(),
                    description.as_str().into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
            (Some(location), None) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = ?, status = ?, description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    location.as_str().into(),
                    job.status.into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
            (None, Some(description)) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = NULL, status = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    job.status.into(),
                    description.as_str().into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
            (None, None) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = NULL, status = ?, description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    job.status.into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
        }
    }

    let result = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    match result {
        Some(mut updated_job) => {
            normalize_job_id(&mut updated_job);
            Response::from_json(&updated_job)
        }
        None => Response::error("Job not found", 404),
    }
}

async fn delete_job(db: &D1Database, id: String) -> Result<Response> {
    let exists = db
        .prepare("SELECT id FROM jobs WHERE id = ?")
        .bind(&[id.clone().into()])?
        .first::<serde_json::Value>(None)
        .await?;

    if exists.is_none() {
        return Response::error("Job not found", 404);
    }

    db.prepare("DELETE FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .run()
        .await?;

    Response::ok("Job deleted successfully")
}
