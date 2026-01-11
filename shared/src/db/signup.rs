use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct IdOnly {
    pub id: i32
}
