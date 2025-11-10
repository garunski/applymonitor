use crate::common::db::get_d1;
use worker::*;

pub async fn handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db_result = get_d1(&ctx.env);

    let status = match db_result {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let health_response = serde_json::json!({
        "status": status,
        "service": "api"
    });

    Response::from_json(&health_response)
}
