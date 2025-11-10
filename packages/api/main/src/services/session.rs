use anyhow::{anyhow, Result};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub sub: String, // user ID
    pub exp: i64,    // expiration timestamp (unix timestamp in seconds) - set by jwt-simple
    pub iss: String, // issuer
}

pub fn make_session_token(user_id: &str, signing_key: &str, issuer: &str) -> Result<String> {
    // Create claims (exp will be set by jwt-simple based on duration)
    let claims = SessionClaims {
        sub: user_id.to_string(),
        exp: 0, // Will be set by jwt-simple
        iss: issuer.to_string(),
    };

    // Create HMAC key from signing key
    let key = HS256Key::from_bytes(signing_key.as_bytes());

    // Create claims with expiration (jwt-simple uses its own Duration type)
    let mut custom_claims =
        Claims::with_custom_claims(claims, jwt_simple::prelude::Duration::from_hours(24));
    custom_claims.issuer = Some(issuer.to_string());
    custom_claims.subject = Some(user_id.to_string());

    let token = key
        .authenticate(custom_claims)
        .map_err(|e| anyhow!("Failed to encode session token: {}", e))?;

    Ok(token)
}

pub fn verify_session_token(token: &str, signing_key: &str) -> Result<SessionClaims> {
    // Create HMAC key from signing key
    let key = HS256Key::from_bytes(signing_key.as_bytes());

    // Verify and decode token (jwt-simple automatically validates expiration)
    let claims = key
        .verify_token::<SessionClaims>(token, None)
        .map_err(|e| anyhow!("Failed to decode session token: {}", e))?;

    Ok(claims.custom)
}
