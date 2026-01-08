use std::sync::{OnceLock, RwLock};

use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    app::dashboard::{conversations::conversations_route, search::search_route}, no_op, utils::{router::{Route, Router, outlet}, session::Session}
};

mod search;
mod conversations;
#[derive(Clone,Copy,PartialEq)]
pub enum Menu {
    Conversations,
    Search
}

struct DashboardStateT {
    active_menu: Menu,
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
                        active_menu: Menu::Conversations,
                    });
                }
            }
            None => {
                DASHBOARD_STATE
                    .set(RwLock::new(Some(DashboardStateT {
                        active_menu: Menu::Conversations,
                    })))
                    .ok()
                    .unwrap();
            }
        };
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
    fn state() -> &'static RwLock<Option<DashboardStateT>> {
        return DASHBOARD_STATE.get().unwrap();
    }
    fn menu() -> Menu {
        let state_lock = DashboardState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.active_menu;
    }
    pub fn set_menu(new_menu: Menu) {
        let state_lock = DashboardState::state();
        let mut state = state_lock.write().unwrap();
        let s = state.as_mut().unwrap();
        s.active_menu = new_menu;
    }
}




fn dashboard() -> Component {
    Layout::get_col_builder().
    children(vec![
        menu_bar(),
        content_area()
    ]).build()
}

fn menu_bar() -> Component {

    let current_menu = DashboardState::menu();

    Layout::get_row_builder()
        .bg_color(Color::LIGHTGRAY)
        .dim((Length::FILL, Length::FILL))
        .flex(4.0)
        .padding((5,5,5,5))
        .gap(5)
        .children(vec![
            TextLayout::get_builder()
                .dim((Length::FIT,Length::FILL))
                .content("Conversations")
                .dbg_name("DBG_LAYOUT")
                .main_align(Alignment::Center)
                .font_size(18)
                .bg_color({
                    if current_menu == Menu::Conversations {
                        Color::GRAY
                    } else {
                        Color::LIGHTGRAY
                    }
                })
                .on_click(Box::new(|_|{
                    Router::push("dashboard/conversations");
                    DashboardState::set_menu(Menu::Conversations);
                    false
                }))
                .padding((5,2,5,2))
                .font_size(24)
                .build(),
            TextLayout::get_builder()
                .dim((Length::FIT,Length::FILL))
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
                .padding((5,2,5,2))
                .on_click(Box::new(|_|{
                    Router::push("dashboard/search");
                    DashboardState::set_menu(Menu::Search);
                    return
                    false
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
        Box::new(||{
            DashboardState::init();
        }),
        Box::new(||{
            DashboardState::de_init();
        }),
        "dashboard_outlet",
        Box::new(|| dashboard()),
        vec![search_route(),conversations_route()],
    )
}
