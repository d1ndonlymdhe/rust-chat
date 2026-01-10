use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash_password: String,
    pub created_at: String,
    pub updated_at: String
}


#[derive(Serialize,Deserialize,Debug)]
pub struct IdOnly {
    pub id: i32
}
