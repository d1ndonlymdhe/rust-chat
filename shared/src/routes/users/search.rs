use serde::{Deserialize, Serialize};

use crate::db::signup::User;


pub struct SearchQuery {
    name: String,
    limit: i64,
    page: i64
}

#[derive(Serialize,Deserialize)]
pub struct SearchUser {
    id: i64,
    username: String
}
impl From<User> for SearchUser {
    fn from(value: User) -> Self {
        return Self { id: value.id, username: value.username }
    }
}


#[derive(Serialize,Deserialize)]
pub struct  SearchUserResult {
    result: Vec<SearchUser>
}

impl SearchUserResult {
    pub fn new(users: Vec<SearchUser>) -> Self {
        return Self {
            result:users
        }
    }
}