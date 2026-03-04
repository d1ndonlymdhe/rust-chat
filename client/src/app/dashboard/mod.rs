use std::{
    sync::{OnceLock, RwLock},
    thread,
};

use shared::routes::{auth::refresh::RefreshResponse, chat::message::ChatMessage, users::search::SearchUser};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    app::dashboard::{conversations::conversations_route, conversations_store::ConversationsState, search::search_route},
    utils::{
        fetch::{ClientModes, Response, fetch}, localstorage::LocalStorage, router::{Route, Router, outlet}, session::Session
    },
};

mod chat_window;
mod conversations;
mod conversations_store;
mod search;
mod search_store;
#[derive(Clone, PartialEq)]
pub enum Menu {
    Conversations { conversation_id: Option<String> },
    Search,
}

struct DashboardStateT {
    active_menu: Menu,
}

fn load_self() {
    let self_loaded = Session::self_loaded();
    let self_loading = Session::self_loading();
    if self_loaded || self_loading {
        return;
    }
    thread::spawn(|| {
        Session::set_self_loading(true);
        let resp = fetch::<()>(ClientModes::GET, "/users/me", &None);
        match resp {
            Ok(resp) => {
                let as_text = resp.text().unwrap();
                let res = serde_json::from_str::<Response<SearchUser>>(&as_text).unwrap();
                if res.success {
                    let user = res.data.unwrap();

                    Session::set_self_details(user.id, user.username);
                    Session::set_self_loaded(true);
                } else {
                    Session::set_self_loaded(false);
                    Router::push("auth/login");
                }
                Session::set_self_loading(false);
            }
            Err(e) => {
                Session::set_self_loading(false);
                Session::set_self_loaded(false);
                let e_str: String = e.into();
                println!("Error loading self details: {}", e_str);
                Router::push("auth/login");
            }
        }
    });
}

static DASHBOARD_STATE: OnceLock<RwLock<Option<DashboardStateT>>> = OnceLock::new();
pub struct DashboardState;
impl DashboardState {
    pub fn init() {
        match DASHBOARD_STATE.get() {
            Some(v) => {
                let has_state = {
                    let state = v.read().unwrap();
                    state.is_some()
                };
                if !has_state {
                    let mut state = v.write().unwrap();
                    state.replace(DashboardStateT {
                        active_menu: Menu::Conversations {
                            conversation_id: None,
                        },
                    });
                }
            }
            None => {
                DASHBOARD_STATE
                    .set(RwLock::new(Some(DashboardStateT {
                        active_menu: Menu::Conversations {
                            conversation_id: None,
                        },
                    })))
                    .ok()
                    .unwrap();
            }
        };
        load_self();
    }
    pub fn de_init() {
        match DASHBOARD_STATE.get() {
            Some(v) => {
                let mut state = v.write().unwrap();
                state.take();
            }
            None => {}
        };
    }
    pub fn is_some() -> bool {
        match DASHBOARD_STATE.get() {
            Some(v) => {
                let state = v.read().unwrap();
                state.is_some()
            }
            None => false,
        }
    }
    fn state() -> &'static RwLock<Option<DashboardStateT>> {
        return DASHBOARD_STATE.get().unwrap();
    }
    fn menu() -> Menu {
        let state_lock = DashboardState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.active_menu.clone();
    }
    pub fn set_menu(new_menu: Menu) {
        let new_path = match &new_menu {
            Menu::Conversations { conversation_id } => match conversation_id {
                Some(conversation_id) => &format!(
                    "dashboard/conversations?conversation_id={}",
                    conversation_id
                ),
                None => "dashboard/conversations",
            },
            Menu::Search => "dashboard/search",
        };

        Router::push(new_path);
        let state_lock = DashboardState::state();
        let mut state = state_lock.write().unwrap();
        let s = state.as_mut().unwrap();
        s.active_menu = new_menu;
    }
}

fn dashboard() -> Component {
    Layout::get_col_builder()
        .children(vec![menu_bar(), content_area()])
        .build()
}

fn menu_bar() -> Component {
    let current_menu = DashboardState::menu();

    Layout::get_row_builder()
        .bg_color(Color::LIGHTGRAY)
        .dim((Length::FILL, Length::FILL))
        .flex(4.0)
        .padding((5, 5, 5, 5))
        .gap(5)
        .children(vec![
            TextLayout::get_builder()
                .dim((Length::FIT, Length::FILL))
                .content("Conversations")
                .dbg_name("DBG_LAYOUT")
                .main_align(Alignment::Center)
                .font_size(18)
                .bg_color({
                    match current_menu {
                        Menu::Conversations { .. } => Color::GRAY,
                        Menu::Search => Color::LIGHTGRAY,
                    }
                })
                .on_click(Box::new(|_| {
                    Router::push("dashboard/conversations");
                    DashboardState::set_menu(Menu::Conversations {
                        conversation_id: None,
                    });
                    false
                }))
                .padding((5, 2, 5, 2))
                .font_size(24)
                .build(),
            TextLayout::get_builder()
                .dim((Length::FIT, Length::FILL))
                .main_align(Alignment::Center)
                .content("Search")
                .font_size(18)
                .bg_color({
                    if current_menu == Menu::Search {
                        Color::GRAY
                    } else {
                        Color::LIGHTGRAY
                    }
                })
                .padding((5, 2, 5, 2))
                .on_click(Box::new(|_| {
                    DashboardState::set_menu(Menu::Search);
                    return false;
                }))
                .font_size(24)
                .build(),
        ])
        .build()
}
fn content_area() -> Component {
    Layout::get_row_builder()
        .flex(96.0)
        .dim((Length::FILL, Length::FILL))
        .children(vec![outlet("dashboard_outlet")])
        .build()
}

pub fn dashboard_route() -> Route {
    Route::container(
        "dashboard",
        Box::new(|| {
            DashboardState::init();
        }),
        Box::new(|| {
            DashboardState::de_init();
        }),
        "dashboard_outlet",
        Box::new(|| dashboard()),
        vec![search_route(), conversations_route()],
    )
}
