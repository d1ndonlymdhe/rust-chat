use rocket::{State, serde::json::Json};
use shared::routes::chat::message::{
    GetMessagesForConversationResponse, Message, SendTextMessagePayload,
};

use crate::{
    ChatMessage, ChatState,
    db::{auth::jwt::Claims, chat::message::add_text_message},
    lib::Response,
};

#[post("/<conversation_id>/messages/send/text", data = "<payload>")]
pub async fn send_text_message(
    pool: &State<sqlx::PgPool>,
    chat_state: &State<ChatState>,
    conversation_id: i32,
    payload: Json<SendTextMessagePayload>,
    claims: Claims,
) -> Response<Message> {
    let msg = add_text_message(pool, conversation_id, claims.user_id, &payload.text).await;

    let s = chat_state.lock().unwrap();

    match msg {
        Ok(message) => {
            s.send_message(
                conversation_id,
                ChatMessage {
                    msg_id: message.id,
                    content: payload.text.clone(),
                    message_type: "text".to_string(),
                    sender_user_id: claims.user_id.clone(),
                    conversation_id: conversation_id,
                    created_at: message.created_at
                },
            );
            Response::success("Message Sent", message)
        }
        Err(e) => {
            let e_string: String = e.to_string();
            error!("Database error while sending message: {}", e_string.clone());
            Response::internal_error(&e_string, None)
        }
    }
}

#[get("/<conversation_id>/messages")]
pub async fn get_messages_for_conversation(
    pool: &State<sqlx::PgPool>,
    conversation_id: i32,
    claims: Claims,
) -> Response<GetMessagesForConversationResponse> {
    let messages_result = crate::db::chat::message::get_messages_for_conversation(
        pool,
        conversation_id,
        claims.user_id,
    )
    .await;
    match messages_result {
        Ok(messages) => Response::success("Messages fetched", messages),
        Err(e) => {
            let e_string: String = e.to_string();
            error!(
                "Database error while fetching messages: {}",
                e_string.clone()
            );
            Response::internal_error(&e_string, None)
        }
    }
}
