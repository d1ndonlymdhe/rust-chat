use serde::{Deserialize, Serialize};
use crate::db::util::SqlJson;
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

#[derive(Serialize,Deserialize)]
pub struct ConversationWithMembers {
    pub id: i32,
    pub title: Option<String>,
    pub conv_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub members: SqlJson<Vec<ConversationMember>>,
}

pub type GetConversationResponse = Vec<ConversationWithMembers>;