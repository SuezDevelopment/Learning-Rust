use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    pub value: f32,
    pub location: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerHealth {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user_id: Uuid,
    pub preferences: serde_json::Value,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(username: String, role: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            role,
            created_at: chrono::Utc::now(),
        }
    }
}
