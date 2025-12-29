use serde::{Serialize,Deserialize};

use crate::db::signup::IdOnly;

#[derive(Deserialize,Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String
}

pub type SignupResponse = IdOnly;