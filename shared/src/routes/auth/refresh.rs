use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String
}
#[derive(Serialize,Deserialize)]
pub struct RefreshResponse {
    pub refresh_token: String,
    pub access_token: String
}