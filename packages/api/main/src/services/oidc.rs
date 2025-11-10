use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use js_sys::Date;
use serde::Deserialize;
use worker::*;

#[derive(Debug, Clone)]
pub struct OIDCProvider {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
struct OIDCDiscovery {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub id_token: String,
}

#[derive(Debug, Deserialize)]
pub struct IDTokenClaims {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub iss: String,
    pub aud: String,
    pub exp: i64,
    pub nonce: Option<String>,
}

impl OIDCProvider {
    pub async fn discover(env: &Env, issuer: &str) -> Result<OIDCProvider> {
        let client_id = env
            .secret("OIDC_GOOGLE_CLIENT_ID")
            .map_err(|_| anyhow!("OIDC_GOOGLE_CLIENT_ID secret not found"))?
            .to_string();

        let client_secret = env
            .secret("OIDC_GOOGLE_CLIENT_SECRET")
            .map_err(|_| anyhow!("OIDC_GOOGLE_CLIENT_SECRET secret not found"))?
            .to_string();

        let discovery_url = format!("{}/.well-known/openid-configuration", issuer);

        let req = Request::new(&discovery_url, Method::Get)?;
        let mut resp = Fetch::Request(req).send().await?;

        if !resp.status_code() == 200 {
            return Err(anyhow!(
                "Failed to fetch OIDC discovery document: status {}",
                resp.status_code()
            ));
        }

        let discovery: OIDCDiscovery = resp.json().await?;

        Ok(OIDCProvider {
            issuer: discovery.issuer,
            authorization_endpoint: discovery.authorization_endpoint,
            token_endpoint: discovery.token_endpoint,
            client_id,
            client_secret,
        })
    }

    pub fn build_authorization_url(&self, redirect_uri: &str, state: &str, nonce: &str) -> String {
        let mut url = url::Url::parse(&self.authorization_endpoint).unwrap();
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("scope", "openid email profile")
            .append_pair("state", state)
            .append_pair("nonce", nonce);
        url.to_string()
    }

    pub async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<TokenResponse> {
        let body = serde_urlencoded::to_string([
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ])?;

        let mut init = RequestInit::new();
        init.with_method(Method::Post);
        init.with_body(Some(body.into()));
        let headers = Headers::new();
        headers.set("Content-Type", "application/x-www-form-urlencoded")?;
        headers.set("Accept", "application/json")?;
        init.with_headers(headers);

        let req = Request::new_with_init(&self.token_endpoint, &init)?;
        let mut resp = Fetch::Request(req).send().await?;

        if resp.status_code() != 200 {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Token exchange failed: status {} - {}",
                resp.status_code(),
                text
            ));
        }

        let token_response: TokenResponse = resp.json().await?;
        Ok(token_response)
    }

    pub async fn validate_id_token(
        &self,
        id_token: &str,
        expected_nonce: &str,
    ) -> Result<IDTokenClaims> {
        // Parse JWT header and payload
        let parts: Vec<&str> = id_token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid ID token format"));
        }

        // Decode payload (without verification for now)
        let payload = parts[1];
        let decoded = general_purpose::URL_SAFE_NO_PAD
            .decode(payload)
            .map_err(|e| anyhow!("Failed to decode ID token payload: {}", e))?;

        let claims: IDTokenClaims = serde_json::from_slice(&decoded)
            .map_err(|e| anyhow!("Failed to parse ID token claims: {}", e))?;

        // Validate issuer
        if claims.iss != self.issuer {
            return Err(anyhow!(
                "Invalid issuer: expected {}, got {}",
                self.issuer,
                claims.iss
            ));
        }

        // Validate audience
        if claims.aud != self.client_id {
            return Err(anyhow!(
                "Invalid audience: expected {}, got {}",
                self.client_id,
                claims.aud
            ));
        }

        // Validate expiration (using js_sys::Date for WASM compatibility)
        let now = (Date::now() / 1000.0) as i64;
        if claims.exp < now {
            return Err(anyhow!("ID token has expired"));
        }

        // Validate nonce
        if let Some(ref nonce) = claims.nonce {
            if nonce != expected_nonce {
                return Err(anyhow!("Invalid nonce"));
            }
        } else {
            return Err(anyhow!("Missing nonce in ID token"));
        }

        // TODO: Verify JWT signature using JWKS
        // For now, we'll trust the token after validating claims
        // In production, you should fetch JWKS and verify the signature

        Ok(claims)
    }
}
