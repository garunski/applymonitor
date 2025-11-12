use worker::*;

mod common;
mod endpoints;
mod services;
mod types;

use common::auth::require_auth;
use common::cors::get_cors;
use endpoints::{auth, email_contacts, health, job_comments, jobs, root};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Create CORS config early so we can use it for error responses too
    let cors = get_cors(&env);

    let path = req.path();
    let method = req.method();

    // Public routes that don't require authentication
    let is_public = matches!(
        (path.as_str(), method.clone()),
        ("/", Method::Get)
            | ("/health", Method::Get)
            | ("/auth/login", Method::Get)
            | ("/auth/callback", Method::Get)
            | ("/auth/logout", Method::Get)
            | ("/auth/register", Method::Post)
            | ("/auth/login/local", Method::Post)
            | ("/auth/password-reset/request", Method::Post)
            | ("/auth/password-reset/confirm", Method::Post)
    );

    // OPTIONS requests (CORS preflight) should always be allowed through without authentication
    let is_options = method == Method::Options;

    // Check authentication for protected routes (but skip OPTIONS requests)
    if !is_public && !is_options {
        match require_auth(&req, &env).await {
            Ok(user_id) => {
                console_log!(
                    "[Main] Auth check passed for route: {} {}, user_id: {}",
                    method,
                    path,
                    user_id
                );
                // User is authenticated, continue
            }
            Err(e) => {
                console_log!(
                    "[Main] Auth check FAILED for route: {} {}, error: {}",
                    method,
                    path,
                    e
                );
                // Return 401 for unauthorized requests (CORS will be applied globally)
                let error_message = format!("Unauthorized: {}", e);
                console_log!("[Main] Returning 401 error: {}", error_message);
                let error_response = Response::error(error_message, 401)?;
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
        // Public auth endpoints (POST)
        .post_async("/auth/register", |req, ctx| async move {
            auth::register(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .post_async("/auth/login/local", |req, ctx| async move {
            auth::login_local(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .post_async("/auth/password-reset/request", |req, ctx| async move {
            auth::request_password_reset(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .post_async("/auth/password-reset/confirm", |req, ctx| async move {
            auth::confirm_password_reset(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .options("/auth/register", |_, _| Response::ok(""))
        .options("/auth/login/local", |_, _| Response::ok(""))
        .options("/auth/password-reset/request", |_, _| Response::ok(""))
        .options("/auth/password-reset/confirm", |_, _| Response::ok(""))
        // Protected API routes
        .get_async(
            "/api/me",
            |req, ctx| async move { auth::me(req, ctx).await },
        )
        .options("/api/me", |_, _| Response::ok(""))
        // Protected auth endpoints
        .get_async("/auth/link", |req, ctx| async move {
            auth::link_provider_endpoint(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .post_async("/auth/unlink", |req, ctx| async move {
            auth::unlink_provider_endpoint(req, ctx)
                .await
                .map_err(|e| worker::Error::RustError(format!("{}", e)))
        })
        .options("/auth/link", |_, _| Response::ok(""))
        .options("/auth/unlink", |_, _| Response::ok(""))
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
        // Job comments routes
        .get_async("/jobs/:id/comments", |req, ctx| async move {
            job_comments::handler(req, ctx).await
        })
        .post_async("/jobs/:id/comments", |req, ctx| async move {
            job_comments::handler(req, ctx).await
        })
        .options("/jobs/:id/comments", |_, _| Response::ok(""))
        // Email contacts routes
        .get_async("/email-contacts", |req, ctx| async move {
            email_contacts::handler(req, ctx).await
        })
        .get_async("/email-contacts/:email", |req, ctx| async move {
            email_contacts::handler(req, ctx).await
        })
        .put_async("/email-contacts/:email", |req, ctx| async move {
            email_contacts::handler(req, ctx).await
        })
        .options("/email-contacts", |_, _| Response::ok(""))
        .options("/email-contacts/:email", |_, _| Response::ok(""))
        .run(req, env)
        .await?;

    // Apply CORS to all responses
    apply_cors(response, &cors)
}

// Helper function to apply CORS headers to any response
fn apply_cors(response: Response, cors: &Cors) -> Result<Response> {
    response.with_cors(cors)
}
