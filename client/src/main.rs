use reqwest::Error;
use serde::Serialize;
use std::sync::{
    Arc, Mutex, OnceLock,
    mpsc::{self, Receiver, Sender},
};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        root::UIRoot,
    },
    raylib::color::Color,
};

use crate::{
    auth::auth_route,
    utils::router::{ROUTER, Route, Router, build_route, outlet},
};

mod auth;
mod utils;

extern crate ui;

pub static BASE_URL: &'static str = "http://localhost:3000";

pub struct HttpClient {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

fn get_tokens() -> (Option<String>, Option<String>) {
    let http_client = &HTTP_CLIENT.get().unwrap().lock().unwrap();
    (
        http_client.access_token.clone(),
        http_client.refresh_token.clone(),
    )
}

enum ClientModes {
    POST,
    GET,
}

fn get_client<Body>(
    mode: ClientModes,
    path: &str,
    body: Option<Body>,
    access_token: Option<String>,
) -> reqwest::blocking::RequestBuilder
where
    Body: Serialize,
{
    let client = match mode {
        ClientModes::POST => reqwest::blocking::Client::new().post(format!("{BASE_URL}{path}")),
        ClientModes::GET => reqwest::blocking::Client::new().get(format!("{BASE_URL}{path}")),
    };

    let client = if let Some(access_token) = access_token {
        client.bearer_auth(access_token)
    } else {
        client
    };

    let client = if let Some(body) = body {
        match mode {
            ClientModes::POST => {
                let req_body = serde_json::to_string(&body).unwrap();
                client.body(req_body)
            }
            ClientModes::GET => client.query(&body),
        }
    } else {
        client
    };
    client
}

pub fn post<Body>(path: &str, body: Option<Body>) -> Result<reqwest::blocking::Response, Error>
where
    Body: Serialize,
{
    let (access_token, refresh_token) = get_tokens();

    let client = get_client(ClientModes::POST, &path, body, access_token);

    let res = client.send()?;
    if res.status().as_u16() == 401 {
        println!("UNAUTHORIZED ATTEMPTING REFRESH");
    }
    return Ok(res);
}

fn init_http_client() {
    let client = reqwest::blocking::Client::new();
    let client = Arc::new(Mutex::new(HttpClient {
        access_token: None,
        refresh_token: None,
    }));
    HTTP_CLIENT.set(client).ok().unwrap();
}

pub static HTTP_CLIENT: OnceLock<Arc<Mutex<HttpClient>>> = OnceLock::new();

pub static UI_REBUILD_SIGNAL_SEND: OnceLock<Sender<()>> = OnceLock::new();

fn init_channel() -> Receiver<()> {
    let (tx, rx) = mpsc::channel();
    UI_REBUILD_SIGNAL_SEND.set(tx).ok().unwrap();
    rx
}

fn no_op() -> Box<dyn Fn() -> ()> {
    return Box::new(|| {});
}

fn main() {
    let ui_rebuild_signal_recv = init_channel();
    ROUTER
        .set(Mutex::new(Router::new("auth/login")))
        .ok()
        .unwrap();
    UIRoot::start(
        Box::new(move || {
            let start = std::time::Instant::now();
            let r = Route::container(
                "root",
                no_op(),
                no_op(),
                "root_outlet",
                Box::new(|| root()),
                vec![auth_route()],
            );

            let path = {
                let router = ROUTER.get().unwrap().lock().unwrap();
                let current_path = router.current_path();
                current_path
            };
            let path_changed = {
                let router = ROUTER.get().unwrap().lock().unwrap();
                router.path_changed()
            };

            let c = build_route(path, r, path_changed);
            let mut router = ROUTER.get().unwrap().lock().unwrap();
            router.reset_path_changed();
            println!("Layout time: {:?}", start.elapsed());
            c
        }),
        (1920, 1000),
        "Hello from lib",
        ui_rebuild_signal_recv,
    );
}

fn root() -> Component {
    Layout::get_row_builder()
        .bg_color(Color::WHEAT)
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(50), Length::FILL))
                .children(vec![outlet("root_outlet")])
                .build(),
        ])
        .build()
}
