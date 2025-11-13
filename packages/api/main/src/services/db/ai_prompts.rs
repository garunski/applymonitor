use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use worker::*;

#[derive(Debug, Serialize)]
pub struct AiPrompt {
    pub id: String,
    pub name: String,
    pub stage: String,
    pub prompt: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_prompt(
    db: &D1Database,
    name: &str,
    stage: &str,
    prompt: &str,
) -> Result<String> {
    use crate::services::password;

    let id = password::generate_uuid()?;

    db.prepare(
        "INSERT INTO ai_prompts (id, name, stage, prompt, is_active) VALUES (?, ?, ?, ?, false)",
    )
    .bind(&[id.clone().into(), name.into(), stage.into(), prompt.into()])?
    .run()
    .await?;

    Ok(id)
}

pub async fn get_active_prompt(db: &D1Database, stage: &str) -> Result<Option<AiPrompt>> {
    let result = db
        .prepare("SELECT id, name, stage, prompt, is_active, created_at, updated_at FROM ai_prompts WHERE stage = ? AND is_active = true LIMIT 1")
        .bind(&[stage.into()])?
        .first::<Value>(None)
        .await?;

    if let Some(row) = result {
        Ok(Some(AiPrompt {
            id: row
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            stage: row
                .get("stage")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            prompt: row
                .get("prompt")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            is_active: row
                .get("is_active")
                .and_then(|v| {
                    v.as_bool()
                        .or_else(|| v.as_u64().map(|n| n != 0))
                        .or_else(|| v.as_f64().map(|n| n != 0.0))
                })
                .unwrap_or(false),
            created_at: row
                .get("created_at")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            updated_at: row
                .get("updated_at")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        }))
    } else {
        Ok(None)
    }
}

pub async fn list_prompts(db: &D1Database, stage: Option<&str>) -> Result<Vec<AiPrompt>> {
    let query = if let Some(_s) = stage {
        "SELECT id, name, stage, prompt, is_active, created_at, updated_at FROM ai_prompts WHERE stage = ? ORDER BY created_at DESC"
    } else {
        "SELECT id, name, stage, prompt, is_active, created_at, updated_at FROM ai_prompts ORDER BY created_at DESC"
    };

    let mut stmt = db.prepare(query);
    if let Some(s) = stage {
        stmt = stmt.bind(&[s.into()])?;
    }

    let result = stmt.all().await?;
    let rows: Vec<Value> = result.results()?;

    let prompts: Vec<AiPrompt> = rows
        .into_iter()
        .map(|row| AiPrompt {
            id: row
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            stage: row
                .get("stage")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            prompt: row
                .get("prompt")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            is_active: row
                .get("is_active")
                .and_then(|v| {
                    v.as_bool()
                        .or_else(|| v.as_u64().map(|n| n != 0))
                        .or_else(|| v.as_f64().map(|n| n != 0.0))
                })
                .unwrap_or(false),
            created_at: row
                .get("created_at")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            updated_at: row
                .get("updated_at")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        })
        .collect();

    Ok(prompts)
}

pub async fn activate_prompt(db: &D1Database, prompt_id: &str, stage: &str) -> Result<()> {
    // First, deactivate all prompts for this stage
    db.prepare("UPDATE ai_prompts SET is_active = false WHERE stage = ?")
        .bind(&[stage.into()])?
        .run()
        .await?;

    // Then activate the specified prompt
    db.prepare(
        "UPDATE ai_prompts SET is_active = true, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(&[prompt_id.into()])?
    .run()
    .await?;

    Ok(())
}
