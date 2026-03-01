#[macro_use]
extern crate rocket;

use crate::routes::{
    auth::{login::login, refresh::refresh, signup::signup},
    chat::{
        conversation::{create_conversation, get_conversations},
        message::{get_messages_for_conversation, send_text_message}, poll::poll_for_chat_events,
    },
    users::search::{search_me, search_users},
};
use dotenvy::dotenv;
use rocket::tokio::sync::mpsc;
use shared::routes::chat::message::ChatMessage;
use sqlx::{PgPool, postgres::PgConnectOptions};
use std::{
    collections::{HashMap, HashSet},
    env,
    str::FromStr,
    sync::{Arc, Mutex},
};

mod db;
mod lib;
mod routes;
#[get("/")]
fn index() -> &'static str {
    return "Hello World";
}



//TODO: The id right now acts as a pointer use lifetimes and references?
#[derive(Debug, Clone)]
pub struct MessageEventSendChannel {
    id: u32,
    channel: mpsc::UnboundedSender<ChatMessage>,
}

pub struct ChatStateManager {
    channel_count: u32,
    peers: HashMap<i32, Vec<MessageEventSendChannel>>,
    conversation_map: HashMap<i32, HashSet<i32>>,
}

pub type ChatState = Arc<Mutex<ChatStateManager>>;

impl ChatStateManager {
    pub fn init() -> Self {
        ChatStateManager {
            channel_count: 0,
            peers: HashMap::new(),
            conversation_map: HashMap::new(),
        }
    }

    pub fn next_channel_id(&mut self) -> u32 {
        self.channel_count += 1;
        return self.channel_count;
    }

    pub fn send_message(&self, conversation_id: i32, message: ChatMessage) {
        let connected_users = self.conversation_map.get(&conversation_id);

        if connected_users.is_none() {
            return;
        }

        let connected_users = connected_users.unwrap();

        for user in connected_users {
            let channels = self.peers.get(user);
            
            if channels.is_none() {
                continue;
            }
        
            let channels = channels.unwrap();
            
            for channel in channels {
                let _ = channel.channel.send(message.clone());
            }
        }
    }

    pub fn add_channel(
        &mut self,
        conversation_ids: &Vec<i32>,
        user_id: i32,
        channel: MessageEventSendChannel,
    ) {
        for conversation_id in conversation_ids {
            self.conversation_map
                .entry(*conversation_id)
                .or_insert_with(HashSet::new)
                .insert(user_id);
        }
        // self.conversation_map.entry(conversation_ids)
        self.peers
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(channel);
    }

    pub fn remove_channel(
        &mut self,
        conversation_ids: &Vec<i32>,
        user_id: i32,
        channel_id: u32,
    ) {
        for conversation_id in conversation_ids {
            let v = self.conversation_map.get_mut(conversation_id);
            if let Some(user_ids_set) = v {
                user_ids_set.remove(&user_id);
            }
        }
        let v = self.peers.get_mut(&user_id);
        if let Some(channels) = v {
            for i in 0..(channels.len() - 1) {
                if channels[i].id == channel_id {
                    channels[i] = channels[i + 1].clone();
                }
            }
            channels.pop();
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let _ = dotenv();
    let db_url = env::var("DATABASE_URL").expect("DB_URL not set");
    let connect_options = PgConnectOptions::from_str(&db_url).unwrap();
    let pool = PgPool::connect_with(connect_options)
        .await
        .expect("Unable to connect to database");
    let chat_manager = ChatStateManager::init();
    let chat_state = Arc::new(Mutex::new(chat_manager));
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    rocket::build()
        .manage(pool)
        .manage(chat_state)
        .mount("/", routes![index])
        .mount("/auth", routes![signup, login, refresh])
        .mount("/users", routes![search_users, search_me])
        .mount(
            "/chat/conversation",
            routes![
                create_conversation,
                get_conversations,
                send_text_message,
                get_messages_for_conversation,
                poll_for_chat_events,
            ],
        )

}
