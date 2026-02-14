use crate::{ChatMessage, db::auth::jwt::Claims};
use rocket::tokio::sync::mpsc;
#[get("/poll_events")]
async fn poll_for_chat_events(claims: Claims){
    let user_id = claims.user_id;
    let (sender,receiver) = mpsc::unbounded_channel::<ChatMessage>();
    // receiver.recv_many(buffer, limit)
}
