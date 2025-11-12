use worker::*;

mod common;
mod handlers;
mod services;

use common::cors::get_cors;
use handlers::{auth, emails, scan};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Create CORS config early so we can use it for error responses too
    let cors = get_cors(&env);

    let router = Router::new();

    let response = router
        .get_async("/status", |req, ctx| async move {
            auth::status(req, ctx.env).await
        })
        .get_async("/auth", |req, ctx| async move {
            auth::initiate_auth(req, ctx.env).await
        })
        .get_async("/gmail/callback", |req, ctx| async move {
            auth::callback(req, ctx.env).await
        })
        .delete_async("/disconnect", |req, ctx| async move {
            auth::disconnect(req, ctx.env).await
        })
        .post_async("/scan", |req, ctx| async move {
            scan::scan_emails(req, ctx.env).await
        })
        .get_async("/scan/:id", |req, ctx| async move {
            let scan_id = ctx
                .param("id")
                .ok_or_else(|| worker::Error::RustError("Invalid scan ID".to_string()))?
                .to_string();
            scan::get_scan(req, ctx.env, scan_id).await
        })
        .get_async("/scans", |req, ctx| async move {
            scan::list_scans(req, ctx.env).await
        })
        .get_async("/emails", |req, ctx| async move {
            emails::list_emails(req, ctx.env).await
        })
        .get_async("/emails/:id", |req, ctx| async move {
            let gmail_id = ctx
                .param("id")
                .ok_or_else(|| worker::Error::RustError("Invalid Gmail ID".to_string()))?
                .to_string();
            emails::get_email(req, ctx.env, gmail_id).await
        })
        .post_async("/emails/:id/assign-job", |req, ctx| async move {
            let gmail_id = ctx
                .param("id")
                .ok_or_else(|| worker::Error::RustError("Invalid Gmail ID".to_string()))?
                .to_string();
            emails::assign_email_to_job(req, ctx.env, gmail_id).await
        })
        .options("/status", |_, _| Response::ok(""))
        .options("/auth", |_, _| Response::ok(""))
        .options("/gmail/callback", |_, _| Response::ok(""))
        .options("/disconnect", |_, _| Response::ok(""))
        .options("/scan", |_, _| Response::ok(""))
        .options("/scan/:id", |_, _| Response::ok(""))
        .options("/scans", |_, _| Response::ok(""))
        .options("/emails", |_, _| Response::ok(""))
        .options("/emails/:id", |_, _| Response::ok(""))
        .options("/emails/:id/assign-job", |_, _| Response::ok(""))
        .run(req, env)
        .await;

    // Apply CORS to all responses, including errors
    match response {
        Ok(resp) => apply_cors(resp, &cors),
        Err(e) => {
            // Create error response with CORS
            let error_resp = Response::error(format!("Internal server error: {:?}", e), 500)?;
            apply_cors(error_resp, &cors)
        }
    }
}

// Helper function to apply CORS headers to any response
fn apply_cors(response: Response, cors: &Cors) -> Result<Response> {
    response.with_cors(cors)
}

#[event(scheduled)]
async fn scheduled(_event: ScheduledEvent, _env: Env, _ctx: ScheduleContext) {
    console_log!("Scheduled event triggered");
}
