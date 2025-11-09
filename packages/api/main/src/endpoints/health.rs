use worker::*;
use crate::common::db::get_d1;
use crate::common::cors::get_cors;

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
    
    let response = Response::from_json(&health_response)?;
    response.with_cors(&get_cors())
}
