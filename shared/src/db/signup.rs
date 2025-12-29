use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hash_password: String,
    pub refresh_token: Option<String>,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize,Deserialize)]
pub struct IdOnly {
    pub id: i64
}
