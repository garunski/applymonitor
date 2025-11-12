use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::password;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let method = req.method();
    let job_id = ctx
        .param("id")
        .ok_or_else(|| worker::Error::RustError("Job ID is required".to_string()))?;

    match method {
        Method::Get => list_comments(&db, job_id.to_string()).await,
        Method::Post => create_comment(&db, req, &ctx.env, job_id.to_string()).await,
        _ => Response::error("Method not allowed", 405),
    }
}

async fn list_comments(db: &D1Database, job_id: String) -> Result<Response> {
    let result = db
        .prepare(
            "SELECT jc.*, u.name, u.email, u.picture FROM job_comments jc 
             LEFT JOIN users u ON jc.user_id = u.id 
             WHERE jc.job_id = ? ORDER BY jc.created_at DESC",
        )
        .bind(&[job_id.into()])?
        .all()
        .await?;

    let comments: Vec<Value> = result.results()?;
    Response::from_json(&comments)
}

async fn create_comment(
    db: &D1Database,
    mut req: Request,
    env: &Env,
    job_id: String,
) -> Result<Response> {
    let user_id = require_auth(&req, env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    // Verify job exists
    let job_exists = db
        .prepare("SELECT id FROM jobs WHERE id = ?")
        .bind(&[job_id.clone().into()])?
        .first::<Value>(None)
        .await?;

    if job_exists.is_none() {
        return Response::error("Job not found", 404);
    }

    // Parse request body
    let body: CreateCommentRequest = req.json().await?;

    if body.content.trim().is_empty() {
        return Response::error("Comment content is required", 400);
    }

    let comment_id = password::generate_uuid()
        .map_err(|e| worker::Error::RustError(format!("Failed to generate UUID: {}", e)))?;

    db.prepare("INSERT INTO job_comments (id, job_id, user_id, content) VALUES (?, ?, ?, ?)")
        .bind(&[
            comment_id.clone().into(),
            job_id.into(),
            user_id.into(),
            body.content.into(),
        ])?
        .run()
        .await?;

    // Fetch the created comment with user info
    let comment_result = db
        .prepare(
            "SELECT jc.*, u.name, u.email, u.picture FROM job_comments jc 
             LEFT JOIN users u ON jc.user_id = u.id 
             WHERE jc.id = ?",
        )
        .bind(&[comment_id.into()])?
        .first::<Value>(None)
        .await?;

    match comment_result {
        Some(comment) => {
            let resp = Response::from_json(&comment)?;
            Ok(resp.with_status(201))
        }
        None => Response::error("Failed to create comment", 500),
    }
}
