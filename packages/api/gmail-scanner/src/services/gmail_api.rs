use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GmailMessage {
    pub id: String,
    pub thread_id: String,
    pub snippet: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmailListResponse {
    pub messages: Option<Vec<GmailMessageListItem>>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessageListItem {
    pub id: String,
    pub thread_id: String,
}

pub async fn list_messages(
    access_token: &str,
    query: Option<&str>,
    max_results: Option<usize>,
    page_token: Option<&str>,
) -> Result<GmailListResponse> {
    let mut url = Url::parse("https://gmail.googleapis.com/gmail/v1/users/me/messages")?;

    if let Some(q) = query {
        url.query_pairs_mut().append_pair("q", q);
    }
    if let Some(max) = max_results {
        url.query_pairs_mut()
            .append_pair("maxResults", &max.to_string());
    }
    if let Some(token) = page_token {
        url.query_pairs_mut().append_pair("pageToken", token);
    }

    let mut request = Request::new(url.as_str(), Method::Get)?;
    request
        .headers_mut()?
        .set("Authorization", &format!("Bearer {}", access_token))?;

    let mut response = Fetch::Request(request).send().await?;
    if response.status_code() != 200 {
        let text = response.text().await?;
        return Err(anyhow!("Gmail API error: {}", text));
    }

    let data: GmailListResponse = response.json().await?;
    Ok(data)
}

pub async fn get_message(access_token: &str, message_id: &str) -> Result<GmailMessage> {
    let mut url = Url::parse(&format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id
    ))?;
    url.query_pairs_mut()
        .append_pair("format", "metadata")
        .append_pair("metadataHeaders", "Subject")
        .append_pair("metadataHeaders", "From")
        .append_pair("metadataHeaders", "To")
        .append_pair("metadataHeaders", "Cc")
        .append_pair("metadataHeaders", "Bcc")
        .append_pair("metadataHeaders", "Date");

    let mut request = Request::new(url.as_str(), Method::Get)?;
    request
        .headers_mut()?
        .set("Authorization", &format!("Bearer {}", access_token))?;

    let mut response = Fetch::Request(request).send().await?;
    if response.status_code() != 200 {
        let text = response.text().await?;
        return Err(anyhow!("Gmail API error: {}", text));
    }

    let data: serde_json::Value = response.json().await?;

    let headers = data["payload"]["headers"]
        .as_array()
        .ok_or_else(|| anyhow!("Missing headers"))?;

    let mut subject = None;
    let mut from = None;
    let mut to = None;
    let mut cc = None;
    let mut bcc = None;
    let mut date = None;

    for header in headers {
        let name = header["name"].as_str().unwrap_or("").to_lowercase();
        let value = header["value"].as_str().map(|s| s.to_string());

        match name.as_str() {
            "subject" => subject = value,
            "from" => from = value,
            "to" => to = value,
            "cc" => cc = value,
            "bcc" => bcc = value,
            "date" => date = value,
            _ => {}
        }
    }

    Ok(GmailMessage {
        id: data["id"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing id"))?
            .to_string(),
        thread_id: data["threadId"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing threadId"))?
            .to_string(),
        snippet: data["snippet"].as_str().unwrap_or("").to_string(),
        subject,
        from,
        to,
        cc,
        bcc,
        date,
    })
}

#[allow(dead_code)]
pub async fn get_message_body(access_token: &str, message_id: &str) -> Result<String> {
    let url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}?format=full",
        message_id
    );

    let mut request = Request::new(&url, Method::Get)?;
    request
        .headers_mut()?
        .set("Authorization", &format!("Bearer {}", access_token))?;

    let mut response = Fetch::Request(request).send().await?;
    if response.status_code() != 200 {
        let text = response.text().await?;
        return Err(anyhow!("Gmail API error: {}", text));
    }

    let data: serde_json::Value = response.json().await?;

    // Extract body from payload
    let payload = data["payload"]
        .as_object()
        .ok_or_else(|| anyhow!("Missing payload"))?;

    // Try to get text/plain or text/html body
    let body = extract_body_from_payload(payload)?;

    Ok(body)
}

#[allow(dead_code)]
fn extract_body_from_payload(
    payload: &serde_json::Map<String, serde_json::Value>,
) -> Result<String> {
    // Check if body is directly in payload (simple message)
    if let Some(body_data) = payload.get("body") {
        if let Some(data) = body_data.get("data") {
            if let Some(data_str) = data.as_str() {
                let decoded = URL_SAFE_NO_PAD
                    .decode(data_str)
                    .map_err(|e| anyhow!("Failed to decode body: {}", e))?;
                return Ok(String::from_utf8_lossy(&decoded).to_string());
            }
        }
    }

    // Check parts (multipart message)
    if let Some(parts) = payload.get("parts").and_then(|p| p.as_array()) {
        for part in parts {
            if let Some(mime_type) = part.get("mimeType").and_then(|m| m.as_str()) {
                if mime_type == "text/plain" || mime_type == "text/html" {
                    if let Some(body_data) = part.get("body") {
                        if let Some(data) = body_data.get("data") {
                            if let Some(data_str) = data.as_str() {
                                let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
                                    .decode(data_str)
                                    .map_err(|e| anyhow!("Failed to decode body: {}", e))?;
                                let text = String::from_utf8_lossy(&decoded).to_string();
                                // Prefer plain text, but use HTML if that's all we have
                                if mime_type == "text/plain" {
                                    return Ok(text);
                                } else if mime_type == "text/html" {
                                    // For now, return HTML as-is. Could strip HTML tags later if needed
                                    return Ok(text);
                                }
                            }
                        }
                    }
                }
                // Recursively check nested parts
                if let Some(nested_parts) = part.get("parts").and_then(|p| p.as_array()) {
                    for nested_part in nested_parts {
                        if let Some(nested_mime) =
                            nested_part.get("mimeType").and_then(|m| m.as_str())
                        {
                            if nested_mime == "text/plain" || nested_mime == "text/html" {
                                if let Some(body_data) = nested_part.get("body") {
                                    if let Some(data) = body_data.get("data") {
                                        if let Some(data_str) = data.as_str() {
                                            let decoded =
                                                base64::engine::general_purpose::URL_SAFE_NO_PAD
                                                    .decode(data_str)
                                                    .map_err(|e| {
                                                        anyhow!("Failed to decode body: {}", e)
                                                    })?;
                                            let text =
                                                String::from_utf8_lossy(&decoded).to_string();
                                            if nested_mime == "text/plain" {
                                                return Ok(text);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(anyhow!("Could not extract body from message"))
}

pub fn build_date_query(start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> String {
    format!(
        "after:{} before:{}",
        start_date.timestamp(),
        end_date.timestamp()
    )
}
