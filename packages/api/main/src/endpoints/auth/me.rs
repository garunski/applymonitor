use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::get_user_by_id;
use serde_json::json;
use worker::*;

pub async fn me(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Note: Auth is already checked in lib.rs before routing, but we check again here
    // for safety. If auth fails here, it means the check in lib.rs was bypassed somehow.
    let user_id = match require_auth(&req, &ctx.env).await {
        Ok(id) => id,
        Err(e) => {
            let error_message = format!("Unauthorized: {}", e);
            return Response::error(error_message, 401);
        }
    };

    let db = get_d1(&ctx.env)?;
    let user = get_user_by_id(&db, &user_id)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get user: {}", e)))?;

    if let Some(user) = user {
        let user_json = json!({
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "picture": user.picture,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
            "providers": user.providers,
        });

        Response::from_json(&user_json)
    } else {
        Response::error("User not found", 404)
    }
}
