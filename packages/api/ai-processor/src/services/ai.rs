use crate::services::db::get_active_prompt;
use crate::services::db::EmailData;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub category: String,
    pub confidence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub recruiter_name: Option<String>,
    pub recruiter_email: Option<String>,
    pub interview_date: Option<String>,
    pub location: Option<String>,
    pub remote: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizationResult {
    pub summary: String,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct AiInput {
    messages: Vec<ChatMessage>,
    max_tokens: u32,
}

pub async fn call_ai(env: &Env, prompt: &str) -> Result<String> {
    // Validate prompt is not empty
    if prompt.trim().is_empty() {
        return Err(anyhow!("Prompt cannot be empty"));
    }

    // Use Cloudflare Workers AI binding
    let ai = env.ai("AI")
        .map_err(|e| anyhow!("Failed to get AI binding: {:?}. Make sure [ai] binding = \"AI\" is configured in wrangler.toml", e))?;

    // Build input with messages and max_tokens, matching TypeScript examples
    let input = AiInput {
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        max_tokens: 1024,
    };

    let output: serde_json::Value = ai
        .run("@cf/meta/llama-3.1-8b-instruct-fast", input)
        .await
        .map_err(|e| anyhow!("AI API error: {:?}", e))?;

    // Extract text from response - try common formats
    let response_text = output
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            output
                .as_object()?
                .get("response")?
                .as_str()
                .map(|s| s.to_string())
        })
        .or_else(|| {
            output
                .as_object()?
                .get("result")?
                .as_str()
                .map(|s| s.to_string())
        })
        .ok_or_else(|| {
            console_log!("Unexpected AI response format: {}", serde_json::to_string_pretty(&output).unwrap_or_default());
            anyhow!("Invalid AI API response format: {:?}", output)
        })?;

    Ok(response_text)
}

fn extract_json_from_text(text: &str) -> Result<String> {
    let start = text.find('{')
        .ok_or_else(|| anyhow!("No JSON object found in response: {}", text))?;
    let end = text.rfind('}')
        .ok_or_else(|| anyhow!("No closing brace found in JSON: {}", text))?;
    Ok(text[start..=end].to_string())
}

fn substitute_variables(template: &str, variables: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

pub async fn classify_email(env: &Env, email: &EmailData) -> Result<ClassificationResult> {
    let prompt_template = get_active_prompt(env, "classify").await?;

    let mut variables = HashMap::new();
    variables.insert("from_email", email.from.as_deref().unwrap_or(""));
    variables.insert("subject", email.subject.as_deref().unwrap_or(""));
    variables.insert(
        "body",
        email
            .body
            .as_deref()
            .or(email.snippet.as_deref())
            .unwrap_or(""),
    );

    let prompt = substitute_variables(&prompt_template, &variables);
    let response = match call_ai(env, &prompt).await {
        Ok(r) => r,
        Err(e) => {
            console_log!("AI call failed: {}", e);
            return Ok(ClassificationResult {
                category: "other".to_string(),
                confidence: 0.0,
            });
        }
    };

    // Extract JSON from response (may contain extra text)
    let json_text = match extract_json_from_text(&response) {
        Ok(j) => j,
        Err(e) => {
            console_log!("Failed to extract JSON: {} - Response: {}", e, response);
            return Ok(ClassificationResult {
                category: "other".to_string(),
                confidence: 0.0,
            });
        }
    };

    // Parse JSON response, default to "other" with low confidence on failure
    match serde_json::from_str::<ClassificationResult>(&json_text) {
        Ok(result) => Ok(result),
        Err(e) => {
            console_log!("Failed to parse classification result: {} - JSON: {}", e, json_text);
            Ok(ClassificationResult {
                category: "other".to_string(),
                confidence: 0.0,
            })
        }
    }
}

pub async fn extract_info(
    env: &Env,
    email: &EmailData,
    category: &str,
) -> Result<ExtractionResult> {
    let prompt_template = get_active_prompt(env, "extract").await?;

    let mut variables = HashMap::new();
    variables.insert("category", category);
    variables.insert("from_email", email.from.as_deref().unwrap_or(""));
    variables.insert("subject", email.subject.as_deref().unwrap_or(""));
    variables.insert(
        "body",
        email
            .body
            .as_deref()
            .or(email.snippet.as_deref())
            .unwrap_or(""),
    );

    let prompt = substitute_variables(&prompt_template, &variables);
    let response = match call_ai(env, &prompt).await {
        Ok(r) => r,
        Err(e) => {
            console_log!("AI call failed: {}", e);
            return Ok(ExtractionResult {
                company: None,
                job_title: None,
                recruiter_name: None,
                recruiter_email: None,
                interview_date: None,
                location: None,
                remote: None,
            });
        }
    };

    // Extract JSON from response (may contain extra text)
    let json_text = match extract_json_from_text(&response) {
        Ok(j) => j,
        Err(e) => {
            console_log!("Failed to extract JSON: {} - Response: {}", e, response);
            return Ok(ExtractionResult {
                company: None,
                job_title: None,
                recruiter_name: None,
                recruiter_email: None,
                interview_date: None,
                location: None,
                remote: None,
            });
        }
    };

    // Parse JSON response, default to empty result on failure
    match serde_json::from_str::<ExtractionResult>(&json_text) {
        Ok(result) => Ok(result),
        Err(e) => {
            console_log!("Failed to parse extraction result: {} - JSON: {}", e, json_text);
            Ok(ExtractionResult {
                company: None,
                job_title: None,
                recruiter_name: None,
                recruiter_email: None,
                interview_date: None,
                location: None,
                remote: None,
            })
        }
    }
}

pub async fn summarize_email(
    env: &Env,
    email: &EmailData,
    category: &str,
) -> Result<SummarizationResult> {
    let prompt_template = get_active_prompt(env, "summarize").await?;

    let mut variables = HashMap::new();
    variables.insert("category", category);
    variables.insert("from_email", email.from.as_deref().unwrap_or(""));
    variables.insert("subject", email.subject.as_deref().unwrap_or(""));
    variables.insert(
        "body",
        email
            .body
            .as_deref()
            .or(email.snippet.as_deref())
            .unwrap_or(""),
    );

    let prompt = substitute_variables(&prompt_template, &variables);
    let response = match call_ai(env, &prompt).await {
        Ok(r) => r,
        Err(e) => {
            console_log!("AI call failed: {}", e);
            return Ok(SummarizationResult {
                summary: "Unable to generate summary".to_string(),
            });
        }
    };

    // Extract JSON from response (may contain extra text)
    let json_text = match extract_json_from_text(&response) {
        Ok(j) => j,
        Err(e) => {
            console_log!("Failed to extract JSON: {} - Response: {}", e, response);
            return Ok(SummarizationResult {
                summary: "Unable to generate summary".to_string(),
            });
        }
    };

    // Parse JSON response, default to unclear summary on failure
    match serde_json::from_str::<SummarizationResult>(&json_text) {
        Ok(result) => Ok(result),
        Err(e) => {
            console_log!("Failed to parse summarization result: {} - JSON: {}", e, json_text);
            Ok(SummarizationResult {
                summary: "Unable to generate summary".to_string(),
            })
        }
    }
}

pub async fn process_email(env: &Env, email_id: &str, user_id: &str) -> Result<()> {
    use crate::common::uuid::generate_uuid;
    use crate::services::db::{get_email_data, save_ai_result, update_email_ai_status};

    // Get email data
    let email = get_email_data(env, email_id).await?;

    // Stage 1: Classify
    let classification = classify_email(env, &email).await?;
    let needs_review = classification.confidence < 0.7;

    // Stage 2: Extract
    let extraction = extract_info(env, &email, &classification.category).await?;

    // Stage 3: Summarize
    let summarization = summarize_email(env, &email, &classification.category).await?;

    // Save results
    let result_id = generate_uuid()?;
    let extracted_data_json = serde_json::to_string(&extraction)?;

    save_ai_result(
        env,
        &result_id,
        email_id,
        user_id,
        Some(&classification.category),
        Some(classification.confidence),
        extraction.company.as_deref(),
        extraction.job_title.as_deref(),
        Some(&summarization.summary),
        Some(&extracted_data_json),
    )
    .await?;

    // Update email status
    update_email_ai_status(env, email_id, true, needs_review).await?;

    Ok(())
}
