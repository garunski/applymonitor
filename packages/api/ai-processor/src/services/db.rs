use crate::common::db::get_d1;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailData {
    pub gmail_id: String,
    pub user_id: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub snippet: Option<String>,
    pub body: Option<String>,
}

pub async fn get_email_data(env: &Env, email_id: &str) -> Result<EmailData> {
    let db = get_d1(env)?;

    #[derive(serde::Deserialize)]
    struct EmailRow {
        gmail_id: String,
        user_id: String,
        subject: Option<String>,
        #[serde(rename = "from")]
        from: Option<String>,
        to: Option<String>,
        snippet: Option<String>,
    }

    let result = db
        .prepare("SELECT gmail_id, user_id, subject, \"from\", \"to\", snippet FROM emails WHERE gmail_id = ?")
        .bind(&[email_id.into()])?
        .first::<EmailRow>(None)
        .await?;

    let row = result.ok_or_else(|| anyhow!("Email not found"))?;

    let email_data = EmailData {
        gmail_id: row.gmail_id,
        user_id: row.user_id,
        subject: row.subject,
        from: row.from,
        to: row.to,
        snippet: row.snippet,
        body: None, // Body will be fetched separately if needed
    };

    Ok(email_data)
}

#[allow(clippy::too_many_arguments)]
pub async fn save_ai_result(
    env: &Env,
    result_id: &str,
    email_id: &str,
    user_id: &str,
    category: Option<&str>,
    confidence: Option<f64>,
    company: Option<&str>,
    job_title: Option<&str>,
    summary: Option<&str>,
    extracted_data: Option<&str>,
) -> Result<()> {
    let db = get_d1(env)?;

    // Handle optional fields - use NULL in SQL for None values
    match (
        category,
        confidence,
        company,
        job_title,
        summary,
        extracted_data,
    ) {
        (Some(cat), Some(conf), Some(comp), Some(title), Some(sum), Some(ext)) => {
            db.prepare(
                "INSERT INTO ai_results (id, email_id, user_id, category, confidence, company, job_title, summary, extracted_data) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&[
                result_id.into(),
                email_id.into(),
                user_id.into(),
                cat.into(),
                conf.into(),
                comp.into(),
                title.into(),
                sum.into(),
                ext.into(),
            ])?
            .run()
            .await?;
        }
        (Some(cat), Some(conf), Some(comp), Some(title), Some(sum), None) => {
            db.prepare(
                "INSERT INTO ai_results (id, email_id, user_id, category, confidence, company, job_title, summary, extracted_data) VALUES (?, ?, ?, ?, ?, ?, ?, ?, NULL)"
            )
            .bind(&[
                result_id.into(),
                email_id.into(),
                user_id.into(),
                cat.into(),
                conf.into(),
                comp.into(),
                title.into(),
                sum.into(),
            ])?
            .run()
            .await?;
        }
        _ => {
            // Handle other combinations - for simplicity, insert with what we have
            let mut query = "INSERT INTO ai_results (id, email_id, user_id".to_string();
            let mut values = "VALUES (?, ?, ?".to_string();
            let mut bindings: Vec<worker::wasm_bindgen::JsValue> =
                vec![result_id.into(), email_id.into(), user_id.into()];

            if let Some(cat) = category {
                query.push_str(", category");
                values.push_str(", ?");
                bindings.push(cat.into());
            } else {
                query.push_str(", category");
                values.push_str(", NULL");
            }

            if let Some(conf) = confidence {
                query.push_str(", confidence");
                values.push_str(", ?");
                bindings.push(conf.into());
            } else {
                query.push_str(", confidence");
                values.push_str(", NULL");
            }

            if let Some(comp) = company {
                query.push_str(", company");
                values.push_str(", ?");
                bindings.push(comp.into());
            } else {
                query.push_str(", company");
                values.push_str(", NULL");
            }

            if let Some(title) = job_title {
                query.push_str(", job_title");
                values.push_str(", ?");
                bindings.push(title.into());
            } else {
                query.push_str(", job_title");
                values.push_str(", NULL");
            }

            if let Some(sum) = summary {
                query.push_str(", summary");
                values.push_str(", ?");
                bindings.push(sum.into());
            } else {
                query.push_str(", summary");
                values.push_str(", NULL");
            }

            if let Some(ext) = extracted_data {
                query.push_str(", extracted_data");
                values.push_str(", ?");
                bindings.push(ext.into());
            } else {
                query.push_str(", extracted_data");
                values.push_str(", NULL");
            }

            query.push_str(") ");
            values.push(')');
            query.push_str(&values);

            db.prepare(&query).bind(&bindings)?.run().await?;
        }
    }

    Ok(())
}

pub async fn update_email_ai_status(
    env: &Env,
    email_id: &str,
    ai_processed: bool,
    needs_review: bool,
) -> Result<()> {
    let db = get_d1(env)?;

    db.prepare("UPDATE emails SET ai_processed = ?, needs_review = ? WHERE gmail_id = ?")
        .bind(&[
            (if ai_processed { 1 } else { 0 }).into(),
            (if needs_review { 1 } else { 0 }).into(),
            email_id.into(),
        ])?
        .run()
        .await?;

    Ok(())
}

pub async fn get_active_prompt(env: &Env, stage: &str) -> Result<String> {
    let db = get_d1(env)?;

    let result = db
        .prepare("SELECT prompt FROM ai_prompts WHERE stage = ? AND is_active = true LIMIT 1")
        .bind(&[stage.into()])?
        .first::<serde_json::Value>(None)
        .await?;

    let row = result.ok_or_else(|| anyhow!("No active prompt found for stage: {}", stage))?;
    let prompt = row
        .get("prompt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Invalid prompt format"))?;
    Ok(prompt.to_string())
}
