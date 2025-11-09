use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::ok("API is running")?
                .with_cors(&Cors::new()
                    .with_origins(["*"])
                    .with_methods([Method::Get, Method::Post, Method::Options])
                    .with_allowed_headers(["Content-Type"]))
        })
        .options("/", |_, _| {
            Response::ok("")?
                .with_cors(&Cors::new()
                    .with_origins(["*"])
                    .with_methods([Method::Get, Method::Post, Method::Options])
                    .with_allowed_headers(["Content-Type"]))
        })
        .get_async("/test", handle_test)
        .options("/test", |_, _| {
            Response::ok("")?
                .with_cors(&Cors::new()
                    .with_origins(["*"])
                    .with_methods([Method::Get, Method::Post, Method::Options])
                    .with_allowed_headers(["Content-Type"]))
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
    response.with_cors(&Cors::new()
        .with_origins(["*"])
        .with_methods([Method::Get, Method::Post, Method::Options])
        .with_allowed_headers(["Content-Type"]))
}

