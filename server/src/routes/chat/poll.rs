use std::time::Duration;

use crate::{
    ChatState, MessageEventSendChannel,
    db::{auth::jwt::Claims, chat::conversation::get_user_conversations},
    lib::Response,
};
use rocket::{
    State,
    tokio::{sync::mpsc, time::timeout},
};
use shared::routes::chat::message::ChatMessage;
use sqlx::PgPool;

#[get("/poll")]
pub async fn poll_for_chat_events(
    pool: &State<PgPool>,
    chat_state: &State<ChatState>,
    claims: Claims,
) -> Response<ChatMessage> {
    let user_id = claims.user_id;
    let conversations = get_user_conversations(pool, user_id).await;
    if let Err(_) = conversations {
        return Response::bad_request("No Conversations", None);
    }
    let conversations = conversations.unwrap();
    let conversation_ids = conversations
        .iter()
        .map(|c| c.id)
        .collect::<Vec<_>>();
    let (sender, mut receiver) = mpsc::unbounded_channel::<ChatMessage>();
    let channel_id;
    {
        let mut s = chat_state.lock().unwrap();
        channel_id = s.next_channel_id();
        s.add_channel(
            &conversation_ids,
            user_id, 
            MessageEventSendChannel {
                id: channel_id,
                channel: sender,
            },
        );
    }
    let received_message = timeout(Duration::from_secs(30), receiver.recv()).await;

    {
        let mut s = chat_state.lock().unwrap();
        s.remove_channel(&conversation_ids, user_id, channel_id);
    }

    if received_message.is_err() {
        return Response::not_found("Timeout", None);
    } else {
        if let Some(received_message) = received_message.unwrap() {
            return Response::success("Message Received", received_message);
        }
    }

    return Response::internal_error("Channel Closed", None);
}
