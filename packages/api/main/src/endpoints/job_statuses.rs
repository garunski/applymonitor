//! Job statuses endpoint handler

use crate::common::db::get_d1;
use crate::services::job_statuses::list_statuses;
use worker::*;

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;
    let method = req.method();

    match method {
        Method::Get => list_statuses(&db).await,
        _ => Response::error("Method not allowed", 405),
    }
}
