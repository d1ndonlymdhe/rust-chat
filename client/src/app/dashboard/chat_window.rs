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
    app::dashboard::conversations_store::ConversationsState,
    utils::{
        fetch::{ClientModes, Response, fetch},
        session::Session,
        state::as_state,
        text_input::{TextInputType, text_input},
    },
};

pub fn chat_window(conversation_id: i32) -> Component {
    Layout::get_col_builder()
        .flex(80.0)
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .children(vec![messages_section(conversation_id), send_message_box()])
        .build()
}

pub fn messages_section(conversation_id: i32) -> Component {
    let messages = ConversationsState::messages(conversation_id);
    let self_id = Session::get_user_id().expect("User id not set");

    let message_components: Vec<Component> = messages
        .iter()
        .enumerate()
        .map(|(idx, m)| message_component(&m.content, m.sender_user_id == self_id, idx))
        .collect();

    Layout::get_col_builder()
        .dbg_name("CHAT_AREA")
        .children(vec![
            Layout::get_col_builder()
                .children(message_components)
                .gap(2)
                .build(),
        ])
        .flex(19.0)
        .build()
}

fn message_component(content: &str, is_self: bool, idx: usize) -> Component {
    Layout::get_col_builder()
        .children(vec![
            TextLayout::get_builder()
                .content(content)
                .font_size(20)
                .bg_color(if is_self {
                    Color::LIGHTGREEN
                } else {
                    Color::SLATEBLUE
                })
                .text_color(if is_self { Color::BLACK } else { Color::WHITE })
                .cross_align(Alignment::Start)
                .main_align(Alignment::Center)
                .dim((Length::FIT, Length::FIT))
                .dbg_name(&format!("MSG {}", idx))
                .padding((5, 2, 5, 2))
                .wrap(true)
                .build(),
        ])
        .dim((Length::FILL, Length::FIT))
        .cross_align(if is_self {
            Alignment::End
        } else {
            Alignment::Start
        })
        .build()
}

pub fn send_message_box() -> Component {
    let draft_message = ConversationsState::message_draft();
    let conversation_id =
        ConversationsState::selected_conversation_id().expect("No conversation selected");
    let message_input = text_input(
        draft_message.clone(),
        as_state(move |new_message| {
            ConversationsState::set_message_draft(new_message.into());
        }),
        TextInputType::Text,
    );

    let input_box = Layout::get_col_builder()
        .flex(8.0)
        .dim((Length::FILL, Length::FILL))
        .children(vec![message_input])
        .build();

    let send_button = TextLayout::get_builder()
        .content("Send")
        .font_size(20)
        .bg_color(Color::DARKGRAY)
        .text_color(Color::WHITE)
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .flex(2.0)
        .on_click(Box::new(move |_| {
            if !draft_message.trim().is_empty() {
                send_message(draft_message.clone(), conversation_id);
                ConversationsState::set_message_draft(String::new());
            }
            true
        }))
        .build();

    Layout::get_row_builder()
        .children(vec![input_box, send_button])
        .dim((Length::FILL, Length::FILL))
        .flex(1.0)
        .build()
}

fn send_message(msg: String, conversation_id: i32) {
    thread::spawn(move || {
        let resp = fetch(
            ClientModes::POST,
            &format!("/chat/conversation/{}/messages/send/text", conversation_id),
            &Some(SendTextMessagePayload { text: msg }),
        );
        match resp {
            Ok(r) => {
                let as_text = r.text().unwrap_or_default();
                println!("Send message response text: {}", as_text);
                let as_json = serde_json::from_str::<Response<Message>>(&as_text);
                match as_json {
                    Ok(msg) => {
                        if !msg.success {
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
    });
}
