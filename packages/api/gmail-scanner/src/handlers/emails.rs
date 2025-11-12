use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use serde::Deserialize;
use serde_json::Value;
use worker::*;

#[derive(Debug, Deserialize)]
pub struct AssignJobRequest {
    pub job_id: Option<String>,
    pub create_job: Option<CreateJobFromEmail>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobFromEmail {
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: Option<String>,
}

pub async fn list_emails(req: Request, env: Env) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&env)?;

    // Get pagination params
    let url = req.url()?;
    let query_params: std::collections::HashMap<String, String> =
        url.query_pairs().into_owned().collect();

    let limit: i32 = query_params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);
    let offset: i32 = query_params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let result = db
        .prepare(
            "SELECT * FROM emails WHERE user_id = ? ORDER BY date DESC, created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(&[user_id.into(), limit.into(), offset.into()])?
        .all()
        .await?;

    let emails: Vec<Value> = result.results()?;
    Response::from_json(&emails)
}

pub async fn get_email(req: Request, env: Env, gmail_id: String) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&env)?;
    let result = db
        .prepare("SELECT * FROM emails WHERE gmail_id = ? AND user_id = ?")
        .bind(&[gmail_id.into(), user_id.into()])?
        .first::<Value>(None)
        .await?;

    match result {
        Some(email) => Response::from_json(&email),
        None => Response::error("Email not found", 404),
    }
}

pub async fn assign_email_to_job(
    mut req: Request,
    env: Env,
    gmail_id: String,
) -> worker::Result<Response> {
    let user_id = require_auth(&req, &env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let body: AssignJobRequest = req.json().await?;
    let db = get_d1(&env)?;

    // Verify email belongs to user
    let email_result = db
        .prepare("SELECT * FROM emails WHERE gmail_id = ? AND user_id = ?")
        .bind(&[gmail_id.clone().into(), user_id.clone().into()])?
        .first::<Value>(None)
        .await?;

    if email_result.is_none() {
        return Response::error("Email not found", 404);
    }

    // If job_id is provided, link email to existing job
    if let Some(job_id) = body.job_id {
        // Verify job exists (we'll add a job_id column to emails table)
        // For now, we'll just return success
        // TODO: Add job_id column to emails table and update it here
        return Response::from_json(&serde_json::json!({
            "success": true,
            "gmail_id": gmail_id,
            "job_id": job_id
        }));
    }

    // If create_job is provided, create new job
    if let Some(create_job) = body.create_job {
        use crate::common::uuid;
        let job_id = uuid::generate_uuid()
            .map_err(|e| worker::Error::RustError(format!("Failed to generate UUID: {}", e)))?;

        let status = create_job.status.unwrap_or_else(|| "open".to_string());

        db.prepare(
            "INSERT INTO jobs (id, title, company, location, status) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&[
            job_id.clone().into(),
            create_job.title.into(),
            create_job.company.into(),
            create_job.location.as_deref().into(),
            status.into(),
        ])?
        .run()
        .await?;

        // TODO: Link email to job when we add job_id column
        return Response::from_json(&serde_json::json!({
            "success": true,
            "gmail_id": gmail_id,
            "job_id": job_id
        }));
    }

    Response::error("Either job_id or create_job must be provided", 400)
}
