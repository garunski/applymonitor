//! Jobs endpoint handler

use crate::common::db::get_d1;
use crate::services::jobs::{
    create_job, delete_job, get_job, get_job_details_data, list_jobs, update_job,
};
use worker::*;

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let method = req.method();

    let job_id = ctx.param("id").map(|s| s.to_string());
    let url = req.url()?;
    let query_params = url
        .query_pairs()
        .collect::<std::collections::HashMap<_, _>>();
    let include_details = query_params
        .get("include")
        .map(|s| s == "details")
        .unwrap_or(false);

    match method {
        Method::Get => {
            if let Some(id) = job_id {
                if include_details {
                    get_job_details_handler(&db, &req, &ctx.env, id).await
                } else {
                    get_job(&db, id).await
                }
            } else {
                list_jobs(&db).await
            }
        }
        Method::Post => create_job(&db, req).await,
        Method::Put => {
            if let Some(id) = job_id {
                update_job(&db, req, id).await
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

async fn get_job_details_handler(
    db: &D1Database,
    req: &Request,
    env: &Env,
    id: String,
) -> Result<Response> {
    let details = get_job_details_data(db, req, env, id).await?;

    let response = serde_json::json!({
        "job": details.job,
        "emails": details.emails,
        "comments": details.comments,
        "timeline_events": details.timeline_events,
        "people": details.people,
        "contacts": details.contacts,
    });

    Response::from_json(&response)
}
