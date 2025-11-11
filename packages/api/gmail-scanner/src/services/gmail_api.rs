use anyhow::{anyhow, Result};
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
    let mut date = None;

    for header in headers {
        let name = header["name"].as_str().unwrap_or("").to_lowercase();
        let value = header["value"].as_str().map(|s| s.to_string());

        match name.as_str() {
            "subject" => subject = value,
            "from" => from = value,
            "to" => to = value,
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
        date,
    })
}

pub fn build_date_query(start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> String {
    format!(
        "after:{} before:{}",
        start_date.timestamp(),
        end_date.timestamp()
    )
}
