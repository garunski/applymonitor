use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::ai_results::get_ai_result;
use worker::*;

pub async fn handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _user_id = require_auth(&req, &ctx.env)
        .await
        .map_err(|e| {
            console_log!("Auth error in ai-results: {}", e);
            worker::Error::RustError(format!("Unauthorized: {}", e))
        })?;

    let email_id = ctx
        .param("email_id")
        .ok_or_else(|| {
            console_log!("Missing email_id parameter");
            worker::Error::RustError("Missing email_id".to_string())
        })?;

    console_log!("Getting AI result for email_id: {}", email_id);

    let db = get_d1(&ctx.env)?;
    let result = get_ai_result(&db, email_id)
        .await
        .map_err(|e| {
            console_log!("Failed to get AI result: {}", e);
            worker::Error::RustError(format!("Failed to get AI result: {}", e))
        })?;

    match result {
        Some(r) => {
            console_log!("Found AI result for email_id: {}", email_id);
            Response::from_json(&r).map_err(|e| {
                console_log!("Failed to serialize AI result: {}", e);
                worker::Error::RustError(format!("Failed to serialize response: {}", e))
            })
        }
        None => {
            console_log!("No AI result found for email_id: {}", email_id);
            Response::error("AI result not found", 404)
        }
    }
}
