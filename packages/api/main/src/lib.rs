use worker::*;

mod common;
mod endpoints;
mod services;
mod types;

use common::auth::require_auth;
use common::cors::get_cors;
use endpoints::{auth, health, jobs, root, test};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Create CORS config early so we can use it for error responses too
    let cors = get_cors(&env);

    let path = req.path();
    let method = req.method();

    // Public routes that don't require authentication
    let is_public = matches!(
        path.as_str(),
        "/" | "/health" | "/auth/login" | "/auth/callback" | "/auth/logout"
    ) && method == Method::Get;

    // OPTIONS requests (CORS preflight) should always be allowed through without authentication
    let is_options = method == Method::Options;

    // Check authentication for protected routes (but skip OPTIONS requests)
    if !is_public && !is_options {
        match require_auth(&req, &env).await {
            Ok(_) => {
                // User is authenticated, continue
            }
            Err(e) => {
                // Return 401 for unauthorized requests (CORS will be applied globally)
                let error_response = Response::error(format!("Unauthorized: {}", e), 401)?;
                return apply_cors(error_response, &cors);
            }
        }
    }

    let router = Router::new();

    let response = router
        // Public routes
        .get_async("/", root::handler)
        .options("/", |_, _| Response::ok(""))
        .get_async("/health", health::handler)
        .options("/health", |_, _| Response::ok(""))
        // Auth routes (public)
        .get_async("/auth/login", |req, ctx| async move {
            auth::login(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .get_async("/auth/callback", |req, ctx| async move {
            auth::callback(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .get_async("/auth/logout", |req, ctx| async move {
            auth::logout(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        // Protected API routes
        .get_async(
            "/api/me",
            |req, ctx| async move { auth::me(req, ctx).await },
        )
        .options("/api/me", |_, _| Response::ok(""))
        // Protected job routes
        .get_async(
            "/jobs",
            |req, ctx| async move { jobs::handler(req, ctx).await },
        )
        .get_async("/jobs/:id", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .post_async(
            "/jobs",
            |req, ctx| async move { jobs::handler(req, ctx).await },
        )
        .put_async("/jobs/:id", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .delete_async("/jobs/:id", |req, ctx| async move {
            jobs::handler(req, ctx).await
        })
        .options("/jobs", |_, _| Response::ok(""))
        .options("/jobs/:id", |_, _| Response::ok(""))
        // Test route (protected)
        .get_async("/test", test::handler)
        .options("/test", |_, _| Response::ok(""))
        .run(req, env)
        .await?;

    // Apply CORS to all responses
    apply_cors(response, &cors)
}

// Helper function to apply CORS headers to any response
fn apply_cors(response: Response, cors: &Cors) -> Result<Response> {
    response.with_cors(cors)
}
