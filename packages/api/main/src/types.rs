use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub provider: String,
    pub provider_sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub created_at: Option<String>,
    pub last_login: Option<String>,
}
