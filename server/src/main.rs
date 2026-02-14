#[macro_use]
extern crate rocket;

use std::{env, str::FromStr};
use rocket::tokio::sync::mpsc;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgConnectOptions};
use crate::routes::{auth::{login::login, refresh::refresh, signup::signup}, chat::{conversation::{create_conversation, get_conversations}, message::{get_messages_for_conversation, send_text_message}}, users::search::{search_me, search_users}};

mod routes;
mod db;
mod lib;
#[get("/")]
fn index() -> &'static str {
    return "Hello World";
}

pub struct ChatMessage {
    content: String,
    sender_user_id: String,
    conversation_id: String,
}

pub struct ChatPeer {
    id: String,
    channel: mpsc::Sender<ChatMessage>,
}


#[launch]
async fn rocket() -> _ {
    let _ = dotenv();
    let db_url = env::var("DATABASE_URL").expect("DB_URL not set");
    let connect_options = PgConnectOptions::from_str(&db_url)
        .unwrap();
    let pool = PgPool::connect_with(connect_options)
        .await
        .expect("Unable to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    rocket::build()
    .manage(pool)
    .mount("/", routes![index])
    .mount("/auth", routes![signup,login,refresh])
    .mount("/users",routes![search_users,search_me])
    .mount("/chat/conversation", routes![create_conversation,get_conversations,send_text_message,get_messages_for_conversation])
}
