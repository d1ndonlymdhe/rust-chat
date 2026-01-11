use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct CreateConversationRequest {
    pub participant_ids: Vec<i32>,
}


#[derive(Serialize,Deserialize)]
pub struct ConversationMember {
    pub user_id: i32,
    pub username: String,
}

#[derive(Serialize,Deserialize)]
pub struct CreateConversationResponse {
    pub conversation_id: String,
    pub title: Option<String>,
    pub members: Vec<ConversationMember>,
}