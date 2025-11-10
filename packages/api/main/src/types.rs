use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String, // UUID
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub providers: Vec<String>, // List of linked providers
}
