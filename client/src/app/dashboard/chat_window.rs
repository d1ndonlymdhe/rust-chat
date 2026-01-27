use std::thread;

use shared::routes::chat::message::{Message, SendTextMessagePayload};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    UI_REBUILD_SIGNAL_SEND, app::dashboard::conversations_store::ConversationsState, utils::{
        fetch::{ClientModes, Response, fetch}, session::Session, state::as_state, text_input::{TextInputType, text_input}
    }
};

pub fn chat_window(conversation_id: i32) -> Component {
    let messages = ConversationsState::messages(conversation_id);
    let self_id = Session::get_user_id().expect("User id not set");
    Layout::get_col_builder()
        .flex(80.0)
        .children({
            let mut children: Vec<Component> = messages
                .iter()
                .map(|m| message_bubble(&m.content, m.sender_member_id == self_id))
                .collect();
            children.push(send_message_box());
            children
        })
        .build()
}

pub fn message_bubble(message: &str, is_sender: bool) -> Component {
    let bg_color = if is_sender {
        Color::LIGHTBLUE
    } else {
        Color::LIGHTGRAY
    };
    let alignment = if is_sender {
        Alignment::End
    } else {
        Alignment::Start
    };
    Layout::get_row_builder()
        .main_align(alignment)
        .children(vec![
            TextLayout::get_builder()
                .content(message)
                .wrap(true)
                .font_size(16)
                .padding((10, 10, 10, 10))
                .bg_color(bg_color)
                .build() as Component,
        ])
        .build()
}

pub fn send_message_box() -> Component {
    let draft_message = ConversationsState::message_draft();
    let conversation_id = ConversationsState::selected_conversation_id().expect("No conversation selected");
    let message_input = text_input(
        draft_message.clone(),
        as_state(move |new_message| {
            ConversationsState::set_message_draft(new_message.into());
        }),
        TextInputType::Text,
    );

    Layout::get_row_builder()
        .bg_color(Color::BEIGE)
        .padding((10, 10, 10, 10))
        .gap(10)
        .children(vec![
            Layout::get_col_builder()
                .flex(95.0)
                .children(vec![message_input])
                .build(),
            TextLayout::get_builder()
                .content("Send")
                .font_size(16)
                .flex(5.0)
                .padding((10, 10, 10, 10))
                .bg_color(Color::LIGHTGREEN)
                .dim((Length::FIT, Length::FIT))
                .on_click(Box::new(move |_| {
                    send_message(
                        draft_message.clone(),
                        conversation_id,
                    );
                    ConversationsState::set_message_draft(String::new());
                    false
                }))
                .build(),
        ])
        .build()
}

fn send_message(msg: String,conversation_id:i32){
    thread::spawn(move || {
        let resp = fetch(
            ClientModes::POST,
            &format!("/chat/conversation/{}/messages/send/text",conversation_id),
            &Some(SendTextMessagePayload{
                text: msg,
            })
        );
        match resp {
            Ok(r) => {
                let as_text = r.text().unwrap_or_default();
                println!("Send message response text: {}", as_text);
                let as_json = serde_json::from_str::<Response<Message>>(&as_text);
                match as_json {
                    Ok(msg) => {
                        if msg.success {
                            ConversationsState::add_message(conversation_id, msg.data.expect("Empty message data").into());
                        } else {
                            eprintln!("Failed to send message: {}", msg.message);
                        }
                    }
                    Err(e) => {
                        let e: String = e.to_string();
                        eprintln!("Failed to parse send message response: {}", e);
                    }
                }
            }
            Err(e) => {
                let e: String = e.into();
                eprintln!("Failed to send message: {}", e);
            }
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    });
}