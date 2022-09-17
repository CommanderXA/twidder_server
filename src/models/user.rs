use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    uuid: Uuid,
    username: String,
    first_name: String,
    last_name: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, first_name: String, last_name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            first_name,
            last_name,
            created_at: Utc::now(),
        }
    }
}
