use serde::{Deserialize, Serialize};

use crate::db::signup::User;


#[derive(Serialize,Deserialize)]
pub struct SearchQuery {
    pub name: String,
    pub limit: i64,
    pub page: i64
}

#[derive(Serialize,Deserialize)]
pub struct SearchUser {
    pub id: i64,
    pub username: String
}
impl From<User> for SearchUser {
    fn from(value: User) -> Self {
        return Self { id: value.id, username: value.username }
    }
}


#[derive(Serialize,Deserialize)]
pub struct  SearchUserResult {
    pub result: Vec<SearchUser>
}

impl SearchUserResult {
    pub fn new(users: Vec<SearchUser>) -> Self {
        return Self {
            result:users
        }
    }
}