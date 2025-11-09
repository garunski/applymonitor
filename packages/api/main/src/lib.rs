use worker::*;

mod common;
mod endpoints;

use endpoints::{root, health, test, jobs};
use common::cors::get_cors;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/", root::handler)
        .options("/", |_, _| {
            Response::ok("")?.with_cors(&get_cors())
        })
        .get_async("/health", health::handler)
        .options("/health", |_, _| {
            Response::ok("")?.with_cors(&get_cors())
        })
        .get_async("/test", test::handler)
        .options("/test", |_, _| {
            Response::ok("")?.with_cors(&get_cors())
        })
        .get_async("/jobs", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .get_async("/jobs/:id", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .post_async("/jobs", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .put_async("/jobs/:id", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .delete_async("/jobs/:id", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .options("/jobs", |_, _| {
            Response::ok("")?.with_cors(&get_cors())
        })
        .options("/jobs/:id", |_, _| {
            Response::ok("")?.with_cors(&get_cors())
        })
        .run(req, env)
        .await
}
