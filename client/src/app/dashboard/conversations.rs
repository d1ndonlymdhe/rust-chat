use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    app::dashboard::{
        chat_window::chat_window,
        conversations_store::{ClientConversation, ConversationsState},
    },
    no_op,
    utils::router::Route,
};

fn conversation_layout() -> Component {
    Layout::get_row_builder()
        .bg_color(Color::BEIGE)
        .children(vec![conversations_list(), messages_section()])
        .build()
}

fn conversations_list() -> Component {
    let conversations = ConversationsState::conversations();
    let loading = ConversationsState::loading();
    let error = ConversationsState::error();
    let selected_conversation_id = ConversationsState::selected_conversation_id();
    Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::LIGHTGRAY)
        .cross_align(Alignment::Start)
        .main_align(Alignment::Start)
        .padding((10, 10, 10, 10))
        .flex(20.0)
        .gap(10)
        .children({
            if let Some(err) = error {
                vec![
                    TextLayout::get_builder()
                        .content(&format!("Error: {}", err))
                        .font_size(20)
                        .build() as Component,
                ]
            } else if loading {
                vec![
                    TextLayout::get_builder()
                        .content("Loading conversations...")
                        .font_size(20)
                        .build() as Component,
                ]
            } else if conversations.is_empty() {
                vec![
                    TextLayout::get_builder()
                        .content("No conversations yet")
                        .font_size(20)
                        .build() as Component,
                ]
            } else {
                conversations
                    .iter()
                    .map(|conv| conversation_list_item(&conv, selected_conversation_id))
                    .collect::<Vec<Component>>()
            }
        })
        .build()
}

fn messages_section() -> Component {
    let selected_conversation_id = ConversationsState::selected_conversation_id();

    match selected_conversation_id {
        Some(conversation_id) => chat_window(conversation_id),
        None => Layout::get_col_builder()
            .dim((Length::FILL, Length::FILL))
            .bg_color(Color::WHITE)
            .main_align(Alignment::End)
            .gap(10)
            .cross_align(Alignment::Center)
            .flex(80.0)
            .children(vec![
                TextLayout::get_builder()
                    .content("Select a conversation to start")
                    .font_size(24)
                    .text_color(Color::GRAY)
                    .build(),
            ])
            .build(),
    }
}

fn conversation_list_item(
    conversation: &ClientConversation,
    selected_id: Option<i32>,
) -> Component {
    let conversation = &conversation.conversation;
    let passed_conversation_id = conversation.id;
    let is_selected = selected_id == Some(conversation.id);

    let conversation_name = if conversation.title.is_none() {
        let conversation_members = conversation
            .members
            .iter()
            .map(|u| u.username.clone())
            .collect::<Vec<_>>();
        conversation_members.join(", ")
    } else {
        conversation.title.clone().unwrap()
    };

    Layout::get_row_builder()
        .dim((Length::FILL, Length::FIT))
        .bg_color(if is_selected {
            Color::SKYBLUE
        } else {
            Color::WHITE
        })
        .cross_align(Alignment::Center)
        // .padding((10, 5, 10, 5))
        .on_click(Box::new(move |_| {
            ConversationsState::set_selected_conversation_id(Some(passed_conversation_id));
            false
        }))
        .children(vec![
            TextLayout::get_builder()
                .content(&conversation_name)
                .padding((10, 5, 10, 5))
                .text_color(Color::BLACK)
                .font_size(20)
                .wrap(true)
                .build(),
        ])
        .build()
}

pub fn conversations_route() -> Route {
    Route::leaf(
        "conversations",
        Box::new(|| {
            ConversationsState::init();
        }),
        // Box::new(|| {
        //     ConversationsState::de_init();
        // }),
        no_op(),
        Box::new(|| conversation_layout()),
    )
}
