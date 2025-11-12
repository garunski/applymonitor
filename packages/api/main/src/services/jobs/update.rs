//! Job update operations

use crate::services::job_statuses::get_status_by_id;
use crate::services::jobs::{normalize_job_id, Job};
use serde_json::Value;
use worker::{D1Database, Request, Response};

/// Update an existing job
pub async fn update_job(
    db: &D1Database,
    mut req: Request,
    id: String,
) -> Result<Response, worker::Error> {
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
        let body: Value = req.json().await?;
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

        // Validate status_id if provided
        let status_id = job.status_id.unwrap_or(100); // Default to 100 (open)
        if get_status_by_id(db, status_id).await?.is_none() {
            return Response::error("Invalid status_id", 400);
        }

        match (&job.location, &job.description) {
            (Some(location), Some(description)) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = ?, status_id = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    location.as_str().into(),
                    status_id.into(),
                    description.as_str().into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
            (Some(location), None) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = ?, status_id = ?, description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    location.as_str().into(),
                    status_id.into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
            (None, Some(description)) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = NULL, status_id = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    status_id.into(),
                    description.as_str().into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
            (None, None) => {
                db.prepare(
                    "UPDATE jobs SET title = ?, company = ?, location = NULL, status_id = ?, description = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                )
                .bind(&[
                    job.title.into(),
                    job.company.into(),
                    status_id.into(),
                    id.clone().into(),
                ])?
                .run()
                .await?;
            }
        }
    }

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
        Some(mut updated_job) => {
            normalize_job_id(&mut updated_job);
            Response::from_json(&updated_job)
        }
        None => Response::error("Job not found", 404),
    }
}
