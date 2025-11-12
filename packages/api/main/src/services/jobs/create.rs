//! Job creation

use crate::services::jobs::Job;
use crate::services::password;
use worker::{D1Database, Request, Response};

/// Create a new job
pub async fn create_job(db: &D1Database, mut req: Request) -> Result<Response, worker::Error> {
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
