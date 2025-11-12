//! Job status service

use crate::services::jobs::JobStatus;
use serde_json::Value;
use worker::{D1Database, Response};

/// List all job statuses ordered by ID
pub async fn list_statuses(db: &D1Database) -> Result<Response, worker::Error> {
    let result = db
        .prepare("SELECT id, name, display_name, description FROM job_statuses ORDER BY id ASC")
        .all()
        .await?;

    let statuses: Vec<Value> = result.results()?;
    Response::from_json(&statuses)
}

/// Get a single job status by ID
pub async fn get_status_by_id(
    db: &D1Database,
    id: i32,
) -> Result<Option<JobStatus>, worker::Error> {
    let result = db
        .prepare("SELECT id, name, display_name, description FROM job_statuses WHERE id = ?")
        .bind(&[id.into()])?
        .first::<Value>(None)
        .await?;

    match result {
        Some(status_value) => {
            let status: JobStatus = serde_json::from_value(status_value)?;
            Ok(Some(status))
        }
        None => Ok(None),
    }
}
