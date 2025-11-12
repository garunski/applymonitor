use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String, // UUID
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub providers: Vec<String>,   // List of linked providers
    pub timezone: Option<String>, // IANA timezone string (e.g., "America/New_York")
    pub is_admin: Option<bool>,   // Admin status
    pub enabled: Option<bool>,    // User enabled/disabled status
}
