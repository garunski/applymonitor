use crate::common::cors::get_cors;
use worker::*;

pub async fn handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response = Response::ok("API is running")?;
    response.with_cors(&get_cors())
}
