use anyhow::{anyhow, Result};
use getrandom::getrandom;

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
