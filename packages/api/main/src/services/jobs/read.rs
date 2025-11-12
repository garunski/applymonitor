//! Job read operations

use crate::services::jobs::normalize_job_id;
use serde_json::Value;
use worker::{D1Database, Response};

/// List all jobs
pub async fn list_jobs(db: &D1Database) -> Result<Response, worker::Error> {
    let result = db
        .prepare(
            "SELECT 
                j.*, 
                j.status_id,
                js.name as status_name
            FROM jobs j
            LEFT JOIN job_statuses js ON j.status_id = js.id
            ORDER BY j.created_at DESC",
        )
        .all()
        .await?;

    let mut jobs: Vec<Value> = result.results()?;
    for job in &mut jobs {
        normalize_job_id(job);
    }
    Response::from_json(&jobs)
}

/// Get a single job by ID
pub async fn get_job(db: &D1Database, id: String) -> Result<Response, worker::Error> {
    let result = db
        .prepare(
            "SELECT 
                j.*, 
                j.status_id,
                js.name as status_name
            FROM jobs j
            LEFT JOIN job_statuses js ON j.status_id = js.id
            WHERE j.id = ?",
        )
        .bind(&[id.into()])?
        .first::<Value>(None)
        .await?;

    match result {
        Some(mut job) => {
            normalize_job_id(&mut job);
            Response::from_json(&job)
        }
        None => Response::error("Job not found", 404),
    }
}
