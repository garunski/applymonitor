use worker::*;

mod common;
mod handlers;
mod services;

use handlers::{auth, scan};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    let response = router
        .get_async("/status", |req, ctx| async move {
            auth::status(req, ctx.env).await
        })
        .get_async("/auth", |req, ctx| async move {
            auth::initiate_auth(req, ctx.env).await
        })
        .get_async("/callback", |req, ctx| async move {
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
                .and_then(|id_str| id_str.parse::<i64>().ok())
                .ok_or_else(|| worker::Error::RustError("Invalid scan ID".to_string()))?;
            scan::get_scan(req, ctx.env, scan_id).await
        })
        .get_async("/scans", |req, ctx| async move {
            scan::list_scans(req, ctx.env).await
        })
        .options("/status", |_, _| Response::ok(""))
        .options("/auth", |_, _| Response::ok(""))
        .options("/callback", |_, _| Response::ok(""))
        .options("/disconnect", |_, _| Response::ok(""))
        .options("/scan", |_, _| Response::ok(""))
        .options("/scan/:id", |_, _| Response::ok(""))
        .options("/scans", |_, _| Response::ok(""))
        .run(req, env)
        .await?;

    Ok(response)
}

#[event(scheduled)]
async fn scheduled(_event: ScheduledEvent, _env: Env, _ctx: ScheduleContext) {
    console_log!("Scheduled event triggered");
}
