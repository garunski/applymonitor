use crate::common::auth::require_admin;
use crate::common::db::get_d1;
use crate::services::db::{get_all_users, update_user_enabled};
use serde_json::json;
use worker::*;

pub async fn list_users(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&ctx.env)?;
    let users = get_all_users(&db)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get users: {}", e)))?;

    let users_json: Vec<serde_json::Value> = users
        .iter()
        .map(|user| {
            json!({
                "id": user.id,
                "email": user.email,
                "name": user.name,
                "picture": user.picture,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
                "providers": user.providers,
                "timezone": user.timezone,
                "is_admin": user.is_admin,
                "enabled": user.enabled,
            })
        })
        .collect();

    Response::from_json(&users_json)
}

pub async fn update_user_enabled_status(
    mut req: Request,
    ctx: RouteContext<()>,
) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let user_id = ctx
        .param("id")
        .ok_or_else(|| worker::Error::RustError("Missing user id".to_string()))?;

    #[derive(serde::Deserialize)]
    struct UpdateRequest {
        enabled: bool,
    }

    let update_data: UpdateRequest = req.json().await?;

    let db = get_d1(&ctx.env)?;
    update_user_enabled(&db, user_id, update_data.enabled)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to update user: {}", e)))?;

    Response::ok("User updated successfully")
}
