//! Job types

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Option<String>,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
