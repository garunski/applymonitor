use crate::common::auth::require_admin;
use crate::common::db::get_d1;
use crate::services::db::ai_prompts::{activate_prompt, create_prompt, list_prompts};
use serde_json::json;
use worker::*;

pub async fn list(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&ctx.env)?;
    let url = req.url()?;
    let query_params = url
        .query_pairs()
        .collect::<std::collections::HashMap<_, _>>();
    let stage = query_params.get("stage").map(|s| s.to_string());

    let prompts = list_prompts(&db, stage.as_deref())
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to list prompts: {}", e)))?;

    Response::from_json(&prompts)
}

pub async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    #[derive(serde::Deserialize)]
    struct CreateRequest {
        name: String,
        stage: String,
        prompt: String,
    }

    let create_data: CreateRequest = req.json().await?;

    let db = get_d1(&ctx.env)?;
    let id = create_prompt(
        &db,
        &create_data.name,
        &create_data.stage,
        &create_data.prompt,
    )
    .await
    .map_err(|e| worker::Error::RustError(format!("Failed to create prompt: {}", e)))?;

    Response::from_json(&json!({
        "id": id,
        "success": true
    }))
}

pub async fn activate(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let prompt_id = ctx
        .param("id")
        .ok_or_else(|| worker::Error::RustError("Missing prompt id".to_string()))?;

    #[derive(serde::Deserialize)]
    struct ActivateRequest {
        stage: String,
    }

    let activate_data: ActivateRequest = req.json().await?;

    let db = get_d1(&ctx.env)?;
    activate_prompt(&db, prompt_id, &activate_data.stage)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to activate prompt: {}", e)))?;

    Response::from_json(&json!({
        "success": true
    }))
}

pub async fn test(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    #[derive(serde::Deserialize)]
    struct TestRequest {
        email_ids: Vec<String>,
        user_id: String,
    }

    let test_data: TestRequest = req.json().await?;

    // Call AI worker to process emails
    let ai_worker_url = ctx
        .env
        .var("AI_WORKER_URL")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "http://localhost:8002".to_string());

    let url = format!("{}/process/batch", ai_worker_url);
    let request_body = json!({
        "email_ids": test_data.email_ids,
        "user_id": test_data.user_id
    });

    let body = serde_json::to_string(&request_body)?;
    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(body.into()));
    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    init.with_headers(headers);
    let request = Request::new_with_init(&url, &init)?;

    let mut response = Fetch::Request(request).send().await?;
    let status = response.status_code();

    if status == 200 {
        let result: serde_json::Value = response.json().await?;
        Response::from_json(&result)
    } else {
        let text = response.text().await?;
        Response::error(format!("AI worker error: {}", text), status)
    }
}
