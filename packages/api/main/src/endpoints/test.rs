use crate::common::db::get_d1;
use worker::*;

pub async fn handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = get_d1(&ctx.env)?;

    let result = db
        .prepare("SELECT name FROM test_items LIMIT 1")
        .first::<serde_json::Value>(None)
        .await?;

    let text = match &result {
        Some(json) => json
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("No name found")
            .to_string(),
        None => "No items found".to_string(),
    };

    Response::ok(text)
}
