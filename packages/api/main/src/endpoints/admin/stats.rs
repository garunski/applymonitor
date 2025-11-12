use crate::common::auth::require_admin;
use crate::common::db::get_d1;
use serde_json::json;
use worker::*;

pub async fn get_stats(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let _admin_id = require_admin(&req, &ctx.env)
        .await
        .map_err(|e| worker::Error::RustError(format!("Unauthorized: {}", e)))?;

    let db = get_d1(&ctx.env)?;

    // Get total users count
    let total_users_result = db
        .prepare("SELECT COUNT(*) as count FROM users")
        .first::<serde_json::Value>(None)
        .await?;

    let total_users = if let Some(row) = total_users_result.as_ref() {
        row.get("count")
            .and_then(|v| v.as_u64())
            .map(|n| n as i32)
            .unwrap_or(0)
    } else {
        0
    };

    // Get enabled users count
    let enabled_users_result = db
        .prepare("SELECT COUNT(*) as count FROM users WHERE enabled = 1")
        .first::<serde_json::Value>(None)
        .await?;

    let enabled_users = if let Some(row) = enabled_users_result.as_ref() {
        row.get("count")
            .and_then(|v| v.as_u64())
            .map(|n| n as i32)
            .unwrap_or(0)
    } else {
        0
    };

    // Get disabled users count
    let disabled_users = total_users - enabled_users;

    // Get admin count
    let admin_count_result = db
        .prepare("SELECT COUNT(*) as count FROM users WHERE is_admin = 1")
        .first::<serde_json::Value>(None)
        .await?;

    let admin_count = if let Some(row) = admin_count_result.as_ref() {
        row.get("count")
            .and_then(|v| v.as_u64())
            .map(|n| n as i32)
            .unwrap_or(0)
    } else {
        0
    };

    let stats = json!({
        "total_users": total_users,
        "enabled_users": enabled_users,
        "disabled_users": disabled_users,
        "admin_count": admin_count,
    });

    Response::from_json(&stats)
}
