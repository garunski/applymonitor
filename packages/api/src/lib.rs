use worker::*;

// CORS configuration for Dioxus web app
fn get_cors() -> Cors {
    Cors::new()
        .with_origins([
            "https://applymonitor.com",
            "https://www.applymonitor.com",
            "http://localhost:9000", // For local development
            "*", // Fallback for development
        ])
        .with_methods([Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options])
        .with_allowed_headers(["Content-Type", "Authorization"])
        .with_max_age(86400)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    // Register routes for api.applymonitor.com
    router
        .get("/", |_, _| {
            Response::ok("API is running")?
                .with_cors(&get_cors())
        })
        .options("/", |_, _| {
            Response::ok("")?
                .with_cors(&get_cors())
        })
        .get_async("/test", handle_test)
        .options("/test", |_, _| {
            Response::ok("")?
                .with_cors(&get_cors())
        })
        // Catch-all route to handle unmatched requests (helps with debugging)
        .get("*", |req, _| {
            let url = req.url().unwrap_or_default();
            Response::ok(format!("Worker invoked! Path: {}, Host: {}", 
                url.pathname(), 
                url.hostname().unwrap_or("unknown")))?
                .with_cors(&get_cors())
        })
        .options("*", |_, _| {
            Response::ok("")?
                .with_cors(&get_cors())
        })
        .run(req, env)
        .await
}

async fn handle_test(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;

    // Query the database for a simple text value
    let result = db.prepare("SELECT name FROM test_items LIMIT 1").first::<serde_json::Value>(None).await?;
    
    let text = match result {
        Some(json) => {
            // Extract the name field from the JSON object
            json.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("No name found")
                .to_string()
        }
        None => "No items found".to_string()
    };

    let response = Response::ok(text)?;
    response.with_cors(&get_cors())
}

