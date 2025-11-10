use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::get_user_by_id;
use serde_json::json;
use worker::*;

pub async fn me(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let origin = req.headers().get("Origin").ok().flatten().unwrap_or_default();
    let cookie_header = req.headers().get("Cookie").ok().flatten().unwrap_or_default();
    console_log!("[API /me] Request received");
    console_log!("[API /me] Origin: {}", origin);
    console_log!("[API /me] Cookie header: {}", cookie_header);
    
    // Note: Auth is already checked in lib.rs before routing, but we check again here
    // for safety. If auth fails here, it means the check in lib.rs was bypassed somehow.
    let user_id = match require_auth(&req, &ctx.env).await {
        Ok(id) => {
            console_log!("[API /me] Auth successful, user_id: {}", id);
            id
        }
        Err(e) => {
            console_log!("[API /me] Auth failed: {}", e);
            console_log!("[API /me] Returning 401 error with message: Unauthorized: {}", e);
            // Return error (CORS will be applied globally)
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
