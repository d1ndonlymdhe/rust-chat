use std::thread;

use shared::routes::users::search::SearchUser;
use ui::components::{common::Component, layout::Layout};

use crate::{
    app::{auth::auth_route, dashboard::dashboard_route},
    no_op,
    utils::{
        fetch::{ClientModes, Response, fetch, refresh_the_token},
        localstorage::LocalStorage,
        router::{Route, Router, outlet},
        session::Session,
    },
};

mod auth;
mod dashboard;
fn app() -> Component {
    Layout::get_col_builder()
        .children(vec![outlet("root_outlet")])
        .build()
}

fn load_session() {
    let token = LocalStorage::get_value("token");
    if let Some(token) = token {
        Session::set_refresh_token(token);
    }
}

fn init() {
    let token = LocalStorage::get_value("token");
    if let Some(token) = token {
        println!("TOKEN = {}",token);
        thread::spawn(move || {
            println!("REFRESHING TOKEN");
            if let Ok(_) = refresh_the_token(Some(token)) {
                println!("REFRESH SUCCESS");
                Router::push("dashboard/conversations");
            } else {
                println!("REFRESH FAIL");
                Router::push("auth/login");
            };
        });
    } else {
        println!("NO TOKEN FOUND");
        Router::push("auth/login");
    }
}

pub fn app_route() -> Route {
    return Route::container(
        "root",
        Box::new(|| init()),
        no_op(),
        "root_outlet",
        Box::new(|| app()),
        vec![auth_route(), dashboard_route()],
    );
}
