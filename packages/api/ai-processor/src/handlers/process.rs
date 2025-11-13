use crate::services::ai::process_email;
use worker::*;

pub async fn process_single(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let email_id = ctx
        .param("email_id")
        .ok_or_else(|| worker::Error::RustError("Missing email_id".to_string()))?;

    // Get user_id from request body or query params
    #[derive(serde::Deserialize)]
    struct ProcessRequest {
        user_id: String,
    }

    let process_req: ProcessRequest = req.json().await?;

    match process_email(&ctx.env, email_id, &process_req.user_id).await {
        Ok(_) => Response::ok("Processing completed"),
        Err(e) => Response::error(format!("Processing failed: {}", e), 500),
    }
}

pub async fn process_batch(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    #[derive(serde::Deserialize)]
    struct BatchRequest {
        email_ids: Vec<String>,
        user_id: String,
    }

    let batch_req: BatchRequest = req.json().await?;

    let mut results = Vec::new();
    for email_id in batch_req.email_ids {
        match process_email(&ctx.env, &email_id, &batch_req.user_id).await {
            Ok(_) => results.push(serde_json::json!({
                "email_id": email_id,
                "status": "success"
            })),
            Err(e) => results.push(serde_json::json!({
                "email_id": email_id,
                "status": "error",
                "error": format!("{}", e)
            })),
        }
    }

    Response::from_json(&serde_json::json!({
        "results": results
    }))
}
