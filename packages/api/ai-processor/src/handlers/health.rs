use worker::*;

pub async fn handler(_req: Request, _env: Env) -> Result<Response> {
    Response::ok("OK")
}
