use crate::common::db::get_d1;
use crate::services::password;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Option<String>,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let method = req.method();

    let job_id = ctx.param("id").map(|s| s.to_string());

    match method {
        Method::Get => {
            if let Some(id) = job_id {
                get_job(&db, id).await
            } else {
                list_jobs(&db).await
            }
        }
        Method::Post => create_job(&db, req).await,
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
}

async fn list_jobs(db: &D1Database) -> Result<Response> {
    let result = db
        .prepare("SELECT * FROM jobs ORDER BY created_at DESC")
        .all()
        .await?;

    let jobs: Vec<serde_json::Value> = result.results()?;
    // Ensure all IDs are strings
    let jobs: Vec<serde_json::Value> = jobs
        .into_iter()
        .map(|mut job| {
            if let Some(id) = job.get_mut("id") {
                if let Some(id_str) = id.as_str() {
                    *id = serde_json::Value::String(id_str.to_string());
                } else if let Some(id_int) = id.as_i64() {
                    *id = serde_json::Value::String(id_int.to_string());
                } else if let Some(id_int) = id.as_u64() {
                    *id = serde_json::Value::String(id_int.to_string());
                }
            }
            job
        })
        .collect();
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
            // Ensure ID is a string
            if let Some(id_val) = job.get_mut("id") {
                if let Some(id_str) = id_val.as_str() {
                    *id_val = serde_json::Value::String(id_str.to_string());
                } else if let Some(id_int) = id_val.as_i64() {
                    *id_val = serde_json::Value::String(id_int.to_string());
                } else if let Some(id_int) = id_val.as_u64() {
                    *id_val = serde_json::Value::String(id_int.to_string());
                }
            }
            Response::from_json(&job)
        }
        None => Response::error("Job not found", 404),
    }
}

async fn create_job(db: &D1Database, mut req: Request) -> Result<Response> {
    let job: Job = req.json().await?;

    if job.title.is_empty() || job.company.is_empty() {
        return Response::error("Title and company are required", 400);
    }

    let job_id = password::generate_uuid()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate UUID: {}", e)))?;

    db.prepare("INSERT INTO jobs (id, title, company, location, status) VALUES (?, ?, ?, ?, ?)")
        .bind(&[
            job_id.clone().into(),
            job.title.into(),
            job.company.into(),
            job.location.as_deref().into(),
            job.status.into(),
        ])?
        .run()
        .await?;

    let created_job = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[job_id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    match created_job {
        Some(mut job) => {
            // Ensure ID is a string (should already be, but just in case)
            if let Some(id_val) = job.get_mut("id") {
                if let Some(id_str) = id_val.as_str() {
                    *id_val = serde_json::Value::String(id_str.to_string());
                } else if let Some(id_int) = id_val.as_i64() {
                    *id_val = serde_json::Value::String(id_int.to_string());
                } else if let Some(id_int) = id_val.as_u64() {
                    *id_val = serde_json::Value::String(id_int.to_string());
                }
            }
            let resp = Response::from_json(&job)?;
            Ok(resp.with_status(201))
        }
        None => Response::error("Failed to retrieve created job", 500),
    }
}

async fn update_job(db: &D1Database, id: String, mut req: Request) -> Result<Response> {
    let job: Job = req.json().await?;

    if job.title.is_empty() || job.company.is_empty() {
        return Response::error("Title and company are required", 400);
    }

    let exists = db
        .prepare("SELECT id FROM jobs WHERE id = ?")
        .bind(&[id.clone().into()])?
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
        job.location.as_deref().into(),
        job.status.into(),
        id.clone().into(),
    ])?
    .run()
    .await?;

    let result = db
        .prepare("SELECT * FROM jobs WHERE id = ?")
        .bind(&[id.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    match result {
        Some(mut updated_job) => {
            // Ensure ID is a string
            if let Some(id_val) = updated_job.get_mut("id") {
                if let Some(id_str) = id_val.as_str() {
                    *id_val = serde_json::Value::String(id_str.to_string());
                } else if let Some(id_int) = id_val.as_i64() {
                    *id_val = serde_json::Value::String(id_int.to_string());
                } else if let Some(id_int) = id_val.as_u64() {
                    *id_val = serde_json::Value::String(id_int.to_string());
                }
            }
            Response::from_json(&updated_job)
        }
        None => Response::error("Failed to retrieve updated job", 500),
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
