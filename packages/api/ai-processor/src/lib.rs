use worker::*;

mod common;
mod handlers;
mod services;

use common::cors::get_cors;
use handlers::{health, process};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let cors = get_cors(&env);

    let router = Router::new();

    let response = router
        .get_async("/health", |req, ctx| async move {
            health::handler(req, ctx.env).await
        })
        .post_async("/process/:email_id", |req, ctx| async move {
            process::process_single(req, ctx).await
        })
        .post_async("/process/batch", |req, ctx| async move {
            process::process_batch(req, ctx).await
        })
        .options("/health", |_, _| Response::ok(""))
        .options("/process/:email_id", |_, _| Response::ok(""))
        .options("/process/batch", |_, _| Response::ok(""))
        .run(req, env)
        .await;

    match response {
        Ok(resp) => apply_cors(resp, &cors),
        Err(e) => {
            let error_resp = Response::error(format!("Internal server error: {:?}", e), 500)?;
            apply_cors(error_resp, &cors)
        }
    }
}

fn apply_cors(response: Response, cors: &Cors) -> Result<Response> {
    response.with_cors(cors)
}
