use rocket::{State, serde::json::Json};
use serde::{Deserialize, Serialize};
use shared::{Response, routes::chat::conversation::{CreateConversationRequest, CreateConversationResponse}};
use sqlx::PgPool;

use crate::db::{auth::jwt::Claims, chat};


#[post("/create", data = "<payload>")]
pub async fn create_conversation(
    pool: &State<PgPool>,
    payload: Json<CreateConversationRequest>,
    claims: Claims,
)->Response<CreateConversationResponse>{
    let CreateConversationRequest { mut participant_ids} = payload.0;
    let Claims{user_id,..} = claims;
    participant_ids.push(user_id);
    let new_conversation_id = chat::conversation::create_conversation(pool, None, participant_ids).await;
    match new_conversation_id {
        Ok(create_response) => {
            return Response::success("Conversation Created", create_response);
        },
        Err(e) => {
            match e  {
                chat::conversation::CreateConversationError::InvalidUsers => {
                    error!("Invalid user IDs provided while creating conversation");
                    return Response::bad_request("One or more user IDs are invalid", None);
                },
                chat::conversation::CreateConversationError::Sqlx(error) => {
                    let e_string: String = error.to_string();
                    error!("Database error while creating conversation: {}", e_string.clone());
                    return Response::internal_error(&e_string, None);
                },
            }
        },
    }

}
