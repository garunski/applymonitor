use worker::*;
use crate::common::db::get_d1;
use crate::common::cors::get_cors;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Option<i64>,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub async fn handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let method = req.method();

    let job_id = ctx.param("id")
        .and_then(|id_str| id_str.parse::<i64>().ok());

    match method {
        Method::Get => {
            if let Some(id) = job_id {
                get_job(&db, id).await
            } else {
                list_jobs(&db).await
            }
        }
        Method::Post => {
            create_job(&db, req).await
        }
        Method::Put => {
            if let Some(id) = job_id {
                update_job(&db, id, req).await
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
    .and_then(|resp| resp.with_cors(&get_cors()))
}

async fn list_jobs(db: &D1Database) -> Result<Response> {
    let result = db
        .prepare("SELECT * FROM jobs ORDER BY created_at DESC")
        .all()
        .await?;

    let jobs: Vec<serde_json::Value> = result.results()?;
    Response::from_json(&jobs)
}

async fn get_job(db: &D1Database, id: i64) -> Result<Response> {
    let result = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    match result {
        Some(job) => Response::from_json(&job),
        None => Response::error("Job not found", 404),
    }
}

async fn create_job(db: &D1Database, mut req: Request) -> Result<Response> {
    let job: Job = req.json().await?;

    if job.title.is_empty() || job.company.is_empty() {
        return Response::error("Title and company are required", 400);
    }

    db.prepare(
        "INSERT INTO jobs (title, company, location, status) VALUES (?, ?, ?, ?)"
    )
    .bind(&[
        job.title.into(),
        job.company.into(),
        job.location.as_ref().map(|s| s.as_str()).into(),
        job.status.into(),
    ])?
    .run()
    .await?;

    let result = db
        .prepare("SELECT last_insert_rowid() as id")
        .first::<serde_json::Value>(None)
        .await?;

    if let Some(row) = result {
        if let Some(id_value) = row.get("id") {
            if let Some(id) = id_value.as_i64() {
                let created_job = db
                    .prepare("SELECT * FROM jobs WHERE id = ?")
                    .bind(&[id.into()])?
                    .first::<serde_json::Value>(None)
                    .await?;

                match created_job {
                    Some(job) => {
                        let resp = Response::from_json(&job)?;
                        return Ok(resp.with_status(201));
                    }
                    None => return Response::error("Failed to retrieve created job", 500),
                }
            }
        }
    }

    Response::error("Failed to create job", 500)
}

async fn update_job(db: &D1Database, id: i64, mut req: Request) -> Result<Response> {
    let job: Job = req.json().await?;

    if job.title.is_empty() || job.company.is_empty() {
        return Response::error("Title and company are required", 400);
    }

    let exists = db
        .prepare("SELECT id FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    if exists.is_none() {
        return Response::error("Job not found", 404);
    }

    db.prepare(
        "UPDATE jobs SET title = ?, company = ?, location = ?, status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(&[
        job.title.into(),
        job.company.into(),
        job.location.as_ref().map(|s| s.as_str()).into(),
        job.status.into(),
        id.into(),
    ])?
    .run()
    .await?;

    let result = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    match result {
        Some(updated_job) => Response::from_json(&updated_job),
        None => Response::error("Failed to retrieve updated job", 500),
    }
}

async fn delete_job(db: &D1Database, id: i64) -> Result<Response> {
    let exists = db
        .prepare("SELECT id FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
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
