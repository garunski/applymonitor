use crate::common::auth::require_auth;
use crate::common::db::get_d1;
use crate::services::db::{get_user_by_id, update_user_timezone};
use chrono_tz::Tz;
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use worker::*;

#[derive(Debug, Deserialize)]
struct TimezoneRequest {
    timezone: Option<String>,
}

pub async fn handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let user_id = match require_auth(&req, &ctx.env).await {
        Ok(id) => id,
        Err(e) => {
            let error_message = format!("Unauthorized: {}", e);
            return Response::error(error_message, 401);
        }
    };

    if req.method() != Method::Put {
        return Response::error("Method not allowed", 405);
    }

    let body: TimezoneRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid JSON: {}", e)))?;

    // Validate timezone if provided
    if let Some(ref tz_str) = body.timezone {
        if Tz::from_str(tz_str).is_err() {
            return Response::error(
                format!(
                    "Invalid timezone: {}. Must be a valid IANA timezone string.",
                    tz_str
                ),
                400,
            );
        }
    }

    let db = get_d1(&ctx.env)?;

    // Update timezone
    update_user_timezone(&db, &user_id, body.timezone.as_deref())
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to update timezone: {}", e)))?;

    // Fetch updated user
    let user = get_user_by_id(&db, &user_id)
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to get user: {}", e)))?
        .ok_or_else(|| worker::Error::RustError("User not found".to_string()))?;

    let user_json = json!({
        "id": user.id,
        "email": user.email,
        "name": user.name,
        "picture": user.picture,
        "created_at": user.created_at,
        "updated_at": user.updated_at,
        "providers": user.providers,
        "timezone": user.timezone,
    });

    Response::from_json(&user_json)
}
