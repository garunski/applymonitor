use worker::*;

pub fn get_cors(env: &Env) -> Cors {
    // Try to get CORS_ORIGIN first, then fall back to FRONTEND_URL, then default
    let origin = env
        .var("CORS_ORIGIN")
        .or_else(|_| env.var("FRONTEND_URL"))
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "http://localhost:9000".to_string());

    Cors::new()
        .with_origins([origin.as_str()])
        .with_methods([
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ])
        .with_allowed_headers(["Content-Type", "Authorization", "Cookie"])
        .with_max_age(86400)
}
