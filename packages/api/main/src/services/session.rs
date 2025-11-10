use anyhow::{anyhow, Result};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    // Note: sub and iss are set via standard JWT claims, not in custom claims
    // to avoid duplicate fields
}

pub fn make_session_token(user_id: &str, signing_key: &str, issuer: &str) -> Result<String> {
    // Create empty custom claims (we only use standard JWT claims: sub, iss, exp)
    let claims = SessionClaims {};

    // Create HMAC key from signing key
    let key = HS256Key::from_bytes(signing_key.as_bytes());

    // Create claims with expiration (jwt-simple uses its own Duration type)
    let mut custom_claims =
        Claims::with_custom_claims(claims, jwt_simple::prelude::Duration::from_hours(24));
    // Set standard JWT claims (sub and iss are standard fields, not custom)
    custom_claims.issuer = Some(issuer.to_string());
    custom_claims.subject = Some(user_id.to_string());

    let token = key
        .authenticate(custom_claims)
        .map_err(|e| anyhow!("Failed to encode session token: {}", e))?;

    Ok(token)
}

pub fn verify_session_token(token: &str, signing_key: &str) -> Result<String> {
    // Create HMAC key from signing key
    let key = HS256Key::from_bytes(signing_key.as_bytes());

    // Verify and decode token (jwt-simple automatically validates expiration)
    let claims = key
        .verify_token::<SessionClaims>(token, None)
        .map_err(|e| anyhow!("Failed to decode session token: {}", e))?;

    // Extract user_id from standard JWT subject claim
    let user_id = claims
        .subject
        .ok_or_else(|| anyhow!("Token missing subject (sub) claim"))?;

    Ok(user_id)
}
