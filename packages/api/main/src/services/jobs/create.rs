//! Job creation

use crate::services::job_statuses::get_status_by_id;
use crate::services::jobs::Job;
use crate::services::password;
use worker::{D1Database, Request, Response};

/// Create a new job
pub async fn create_job(db: &D1Database, mut req: Request) -> Result<Response, worker::Error> {
    let job: Job = req.json().await?;

    if job.title.is_empty() || job.company.is_empty() {
        return Response::error("Title and company are required", 400);
    }

    // Validate status_id if provided, otherwise default to 100 (open)
    let status_id = job.status_id.unwrap_or(100);
    if get_status_by_id(db, status_id).await?.is_none() {
        return Response::error("Invalid status_id", 400);
    }

    let job_id = password::generate_uuid()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate UUID: {}", e)))?;

    match (&job.location, &job.description) {
        (Some(location), Some(description)) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status_id, description) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                location.as_str().into(),
                status_id.into(),
                description.as_str().into(),
            ])?
            .run()
            .await?;
        }
        (Some(location), None) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status_id, description) VALUES (?, ?, ?, ?, ?, NULL)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                location.as_str().into(),
                status_id.into(),
            ])?
            .run()
            .await?;
        }
        (None, Some(description)) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status_id, description) VALUES (?, ?, ?, NULL, ?, ?)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                status_id.into(),
                description.as_str().into(),
            ])?
            .run()
            .await?;
        }
        (None, None) => {
            db.prepare(
                "INSERT INTO jobs (id, title, company, location, status_id, description) VALUES (?, ?, ?, NULL, ?, NULL)",
            )
            .bind(&[
                job_id.clone().into(),
                job.title.clone().into(),
                job.company.clone().into(),
                status_id.into(),
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
        "status_id": status_id,
        "description": job.description,
        "created_at": null,
        "updated_at": null
    });

    let resp = Response::from_json(&created_job)?;
    Ok(resp.with_status(201))
}
