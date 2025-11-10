use worker::*;

pub async fn handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("API is running")
}
