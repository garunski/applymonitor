use worker::*;

pub fn get_cors() -> Cors {
    Cors::new()
        .with_origins([
            "https://applymonitor.com",
            "https://www.applymonitor.com",
            "http://localhost:9000",
            "http://127.0.0.1:9000",
            "*",
        ])
        .with_methods([
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ])
        .with_allowed_headers(["Content-Type", "Authorization"])
        .with_max_age(86400)
}
