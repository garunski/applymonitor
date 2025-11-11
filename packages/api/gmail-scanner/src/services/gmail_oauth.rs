use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_urlencoded;
use url::Url;
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub fn build_authorization_url(client_id: &str, redirect_uri: &str, state: &str) -> Result<String> {
    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "https://www.googleapis.com/auth/gmail.readonly")
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent")
        .append_pair("state", state);

    Ok(url.to_string())
}

pub async fn exchange_code(
    code: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> Result<TokenResponse> {
    let body = serde_urlencoded::to_string([
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
    ])?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(body.into()));
    let headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;
    headers.set("Accept", "application/json")?;
    init.with_headers(headers);

    let request = Request::new_with_init("https://oauth2.googleapis.com/token", &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let text = response.text().await?;
        return Err(anyhow!("Token exchange failed: {}", text));
    }

    let token_data: serde_json::Value = response.json().await?;

    Ok(TokenResponse {
        access_token: token_data["access_token"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing access_token"))?
            .to_string(),
        refresh_token: token_data["refresh_token"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing refresh_token"))?
            .to_string(),
        expires_in: token_data["expires_in"]
            .as_i64()
            .ok_or_else(|| anyhow!("Missing expires_in"))?,
    })
}

pub async fn refresh_token(
    refresh_token: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<TokenResponse> {
    let body = serde_urlencoded::to_string([
        ("refresh_token", refresh_token),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("grant_type", "refresh_token"),
    ])?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(body.into()));
    let headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;
    headers.set("Accept", "application/json")?;
    init.with_headers(headers);

    let request = Request::new_with_init("https://oauth2.googleapis.com/token", &init)?;
    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let text = response.text().await?;
        return Err(anyhow!("Token refresh failed: {}", text));
    }

    let token_data: serde_json::Value = response.json().await?;

    Ok(TokenResponse {
        access_token: token_data["access_token"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing access_token"))?
            .to_string(),
        refresh_token: refresh_token.to_string(),
        expires_in: token_data["expires_in"]
            .as_i64()
            .ok_or_else(|| anyhow!("Missing expires_in"))?,
    })
}
