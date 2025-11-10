use anyhow::{anyhow, Result};
use argon2::password_hash::{
    rand_core::{CryptoRng, RngCore},
    SaltString,
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::Engine;
use getrandom::getrandom;

/// Custom RNG that uses getrandom (WASM-compatible)
struct GetRandomRng;

impl RngCore for GetRandomRng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        getrandom(&mut bytes).unwrap_or_default();
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        getrandom(&mut bytes).unwrap_or_default();
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom(dest).unwrap_or_default();
    }

    fn try_fill_bytes(
        &mut self,
        dest: &mut [u8],
    ) -> std::result::Result<(), argon2::password_hash::rand_core::Error> {
        getrandom(dest).map_err(|_| argon2::password_hash::rand_core::Error::new("getrandom error"))
    }
}

impl CryptoRng for GetRandomRng {}

/// Hash password using argon2id
pub fn hash_password(password: &str) -> Result<String> {
    // Generate salt using getrandom (WASM-compatible)
    let salt = SaltString::generate(&mut GetRandomRng);
    let argon2 = Argon2::default();

    // Use PasswordHasher trait method now that password-hash feature is enabled
    let password_hash = PasswordHasher::hash_password(&argon2, password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

    Ok(password_hash.to_string())
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| anyhow!("Invalid password hash: {}", e))?;

    let argon2 = Argon2::default();
    // Use PasswordVerifier trait method now that password-hash feature is enabled
    match PasswordVerifier::verify_password(&argon2, password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Check if password hash needs rehashing (for future use when updating argon2 params)
pub fn needs_rehash(_hash: &str) -> bool {
    // For now, always return false
    // In the future, check if hash uses outdated argon2 parameters
    false
}

/// Generate secure random token (32 bytes)
pub fn generate_secure_token() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom(&mut bytes).map_err(|e| anyhow!("Failed to generate random bytes: {}", e))?;

    // Convert to hex string using base64 encoding
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes))
}

/// Hash token for database lookup (deterministic hash)
/// Note: This uses base64 encoding as a simple deterministic representation
/// For production, consider using SHA-256 or similar for better security
pub fn hash_token_for_lookup(token: &str) -> String {
    // For now, use base64 encoding as a simple deterministic hash
    // In production, use SHA-256: sha2::Sha256::digest(token.as_bytes())
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(token.as_bytes())
}

/// Generate UUID v4 (random UUID)
/// Format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
/// where x is any hexadecimal digit and y is one of 8, 9, A, or B
pub fn generate_uuid() -> Result<String> {
    let mut bytes = [0u8; 16];
    getrandom(&mut bytes).map_err(|e| anyhow!("Failed to generate random bytes: {}", e))?;

    // Set version (4) and variant bits
    bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 10

    // Format as UUID string
    let uuid = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    );

    Ok(uuid)
}
