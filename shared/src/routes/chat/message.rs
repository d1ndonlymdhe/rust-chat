use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub sender_member_id: i32,
    pub message_type: String,
    pub message_content_id: i32,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct RawDbMessage {
    pub id: i32,
    pub conversation_id: i32,
    pub sender_member_id: i32,
    pub message_type: String,
    pub message_content_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<RawDbMessage> for Message {
    fn from(raw: RawDbMessage) -> Self {
        Message {
            id: raw.id,
            conversation_id: raw.conversation_id,
            sender_member_id: raw.sender_member_id,
            message_type: raw.message_type,
            message_content_id: raw.message_content_id,
            content: String::new(), // Placeholder, should be populated appropriately
            created_at: raw.created_at,
            updated_at: raw.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SendTextMessagePayload {
    pub text: String,
    pub conversation_id: i32,
}