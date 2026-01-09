use std::{
    sync::{OnceLock, RwLock},
    thread,
};

use reqwest::Method;
use shared::{
    ResponseStruct,
    routes::users::{self, search::{SearchQuery, SearchUserResult}},
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
    UI_REBUILD_SIGNAL_SEND, no_op, utils::{
        fetch::{ClientModes, fetch},
        router::Route,
        state::as_state,
        text_input::{TextInputType, text_input},
    }
};

struct SearchPageStateT {
    search_query: String,
    results: Vec<String>,
    loading: bool,
    error: Option<String>,
}

static SEARCH_PAGE_STATE: OnceLock<RwLock<Option<SearchPageStateT>>> = OnceLock::new();

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
                    let usernames = result.result.into_iter().map(|u|{u.username}).collect::<Vec<String>>();
                    println!("Search result text");
                    println!("{text}");
                    SearchState::set_results(usernames);
                    
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

struct SearchState;
impl SearchState {
    pub fn init() {
        match SEARCH_PAGE_STATE.get() {
            Some(v) => {
                let has_state = {
                    let state = v.read().unwrap();
                    state.is_some()
                };
                if !has_state {
                    let mut state = v.write().unwrap();
                    state.replace(SearchPageStateT {
                        search_query: "".into(),
                        results: vec![],
                        loading: false,
                        error: None,
                    });
                }
            }
            None => {
                SEARCH_PAGE_STATE
                    .set(RwLock::new(Some(SearchPageStateT {
                        search_query: "".into(),
                        results: vec![],
                        loading: false,
                        error: None
                    })))
                    .ok()
                    .unwrap();
            }
        }
    }
    pub fn de_init() {
        match SEARCH_PAGE_STATE.get() {
            Some(v) => {
                let mut state = v.write().unwrap();
                state.take();
            }
            None => {}
        }
    }
    fn state() -> &'static RwLock<Option<SearchPageStateT>> {
        SEARCH_PAGE_STATE
            .get()
            .expect("Search Page State not initialized")
    }
    pub fn set_search_query(new_query: String) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.search_query = new_query;
    }
    pub fn set_loading(is_loading: bool) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.loading = is_loading;
    }
    pub fn set_results(new_results: Vec<String>) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.results = new_results;
    }
    pub fn set_error(new_error: Option<String>) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.error = new_error;
    }
    pub fn search_query() -> String {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.search_query.clone()
    }
    pub fn results() -> Vec<String> {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.results.clone()
    }
    pub fn loading() -> bool {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.loading
    }
    pub fn error() -> Option<String> {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.error.clone()
    }
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
                        results
                            .iter()
                            .map(|res| {
                                TextLayout::get_builder().content(res).font_size(20).build()
                                    as Component
                            })
                            .collect()
                    }
                }
            }
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
