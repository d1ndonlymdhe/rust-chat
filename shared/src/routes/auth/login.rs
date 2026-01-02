use serde::{Deserialize, Serialize};

use crate::routes::auth::signup::SignupRequest;

pub type LoginRequest = SignupRequest;


#[derive(Serialize,Deserialize)]
pub struct LoginResponse {
    pub refresh_token: String,
    pub access_token: String
}