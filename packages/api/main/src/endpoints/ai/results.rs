use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::ai_results::get_ai_result;
use worker::*;

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _user_id = require_auth(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let email_id = ctx
        .param("email_id")
        .ok_or_else(|| worker::Error::RustError("Missing email_id".to_string()))?;

    let db = get_d1(&ctx.env)?;
    let result = get_ai_result(&db, email_id)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get AI result: {}", e)))?;

    match result {
        Some(r) => Response::from_json(&r),
        None => Response::error("AI result not found", 404),
    }
}
