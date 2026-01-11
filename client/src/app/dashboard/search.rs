use std::thread;

use shared::{
    ResponseStruct,
    routes::{chat::conversation::{CreateConversationRequest, CreateConversationResponse}, users::search::{SearchQuery, SearchUserResult}},
};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    UI_REBUILD_SIGNAL_SEND,
    utils::{
        fetch::{ClientModes, fetch},
        router::Route,
        state::as_state,
        text_input::{TextInputType, text_input},
    },
};

use super::search_store::SearchState;

fn execute_search() {
    let query = SearchState::search_query();
    if query.trim().is_empty() {
        return;
    }
    SearchState::set_loading(true);
    thread::spawn(move || {
        let res = fetch(
            ClientModes::GET,
            "/users/search",
            &Some(SearchQuery {
                name: query.to_string(),
                limit: 20,
                page: 1,
            }),
        );
        match res {
            Ok(response) => {
                let text = response.text().unwrap();
                let res_json =
                    serde_json::from_str::<ResponseStruct<SearchUserResult>>(&text).unwrap();
                if res_json.success {
                    let result = res_json.data.unwrap();
                    println!("Search result text");
                    println!("{text}");
                    SearchState::set_results(result.result);
                } else {
                    SearchState::set_results(vec![]);
                }
            }
            Err(_) => {
                SearchState::set_results(vec![]);
            }
        }
        SearchState::set_loading(false);
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    });
}

fn create_conversation_with_user(user_id: i32) {
    thread::spawn(move || {
        let res = fetch(
            ClientModes::POST,
            "/chat/conversation/create",
            &Some(
                CreateConversationRequest{
                    participant_ids: vec![user_id],
                }
            )
        );
        match res {
            Ok(response) => {
                let text = response.text().unwrap();
                let res_json =
                    serde_json::from_str::<ResponseStruct<CreateConversationResponse>>(&text).unwrap();
                if res_json.success {
                    let conversation_details = res_json.data.unwrap();
                    println!("Created conversation with ID: {}", conversation_details.conversation_id);
                    println!("Created conversation between users: {:?}", conversation_details.members.iter().map(|v|{v.username.clone()}).collect::<Vec<_>>());
                } else {
                    SearchState::set_error(Some(res_json.message));
                }
            }
            Err(e) => {
                SearchState::set_error(Some(e.into()));
            }
        }
    });
}

fn search_layout() -> Component {
    Layout::get_col_builder()
        .bg_color(Color::BEIGE)
        .cross_align(Alignment::Center)
        .children(vec![search_bar(), search_results()])
        .build()
}

fn search_bar() -> Component {
    let search_query = SearchState::search_query();
    let loading = SearchState::loading();
    Layout::get_row_builder()
        .padding((0, 10, 0, 0))
        .dim((Length::FillPer(70), Length::FILL))
        .flex(5.0)
        .gap(10)
        .children(vec![
            Layout::get_row_builder()
                .dim((Length::FILL, Length::FILL))
                .children(vec![text_input(
                    search_query,
                    as_state(|new_query| {
                        SearchState::set_search_query(new_query.into());
                    }),
                    TextInputType::Text,
                )])
                .flex(92.0)
                .build(),
            TextLayout::get_builder()
                .dim((Length::FILL, Length::FILL))
                .cross_align(Alignment::Center)
                .main_align(Alignment::Center)
                .content("Search")
                .on_click(Box::new(move |_|{
                    if loading {
                        return false;
                    }
                    execute_search();
                    false
                }))
                .font_size(24)
                .flex(8.0)
                .bg_color(Color::LIGHTGRAY)
                .build(),
        ])
        .build()
}

fn search_results() -> Component {
    let results = SearchState::results();
    let loading = SearchState::loading();
    let error = SearchState::error();
    Layout::get_col_builder()
        .flex(95.0)
        .cross_align(Alignment::Center)
        .main_align(Alignment::Start)
        .padding((0, 10, 0, 10))
        .children({
            if let Some(err) = error {
                vec![
                    TextLayout::get_builder()
                        .content(&format!("Error: {}", err))
                        .font_size(20)
                        .build() as Component,
                ]
            } else{
                if loading {
                    vec![
                        TextLayout::get_builder()
                            .content("Loading...")
                            .font_size(20)
                            .build() as Component,
                    ]
                } else {
                    if results.is_empty() {
                        vec![
                            TextLayout::get_builder()
                                .content("No results found")
                                .font_size(20)
                                .build() as Component,
                        ]
                    } else {
                        vec![search_results_results()]
                    }
                }
            }
        })
        .build()
}

fn search_results_results() -> Component {

    Layout::get_col_builder()
        .dim((Length::FillPer(30), Length::FILL))
        .cross_align(Alignment::Center)
        .main_align(Alignment::Start)
        .gap(10)
        .children({
            let mut children = vec![
                Layout::get_row_builder()
                    .dim((Length::FILL, Length::FIT))
                    .bg_color(Color::GRAY)
                    .cross_align(Alignment::Center)
                    .overflow_y(false)
                    .children(vec![TextLayout::get_builder()
                        .dim((Length::FILL, Length::FIT))
                        .padding((5, 10, 5, 10))
                        .content("Click to start chatting")
                        .font_size(20)
                        .build()])
                    .build() as Component,
            ];
            let results = SearchState::results();
            children.extend(
                results
                    .into_iter()
                    .map(|res| {
                        Layout::get_row_builder()
                            .dim((Length::FILL, Length::FIT))
                            .bg_color(Color::CYAN)
                            .cross_align(Alignment::Center)
                            .overflow_y(false)
                            .on_click(Box::new(move |_|{
                                let loading = SearchState::loading();
                                if loading {
                                    return false;
                                }
                                create_conversation_with_user(res.id);
                                false
                            }))
                            .children(vec![TextLayout::get_builder()
                                .dim((Length::FILL, Length::FIT))
                                .padding((5, 10, 5, 10))
                                .content(&res.username)
                                .wrap(true)
                                .font_size(50)
                                .build()])
                            .build() as Component
                    })
                    .collect::<Vec<Component>>(),
            );

            children
        })
        .build()
}


pub fn search_route() -> Route {
    Route::leaf(
        "search",
        Box::new(|| {
            SearchState::init();
        }),
        Box::new(|| {
            SearchState::de_init();
        }),
        Box::new(|| search_layout()),
    )
}
