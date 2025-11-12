//! Job deletion

use serde_json::Value;
use worker::{D1Database, Response};

/// Delete a job
pub async fn delete_job(db: &D1Database, id: String) -> Result<Response, worker::Error> {
    let exists = db
        .prepare("SELECT id FROM jobs WHERE id = ?")
        .bind(&[id.clone().into()])?
        .first::<Value>(None)
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
