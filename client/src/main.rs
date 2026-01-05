use reqwest::Error;
use serde::Serialize;
use shared::routes::auth::refresh::{RefreshRequest, RefreshResponse};
use std::sync::{
    OnceLock,
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
    utils::router::{Route, Router, build_route, outlet},
    utils::session::Session,
};

mod auth;
mod utils;

extern crate ui;

pub static BASE_URL: &'static str = "http://localhost:3000";

enum ClientModes {
    POST,
    GET,
}

fn get_client<Body>(
    mode: ClientModes,
    path: &str,
    body: &Option<Body>,
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

fn refresh_the_token(refresh_token: Option<String>) -> Result<(), ()> {
    match refresh_token {
        Some(refresh_token) => {
            let res = reqwest::blocking::Client::new()
                .post(format!("{BASE_URL}/refresh"))
                .body(serde_json::to_string(&RefreshRequest { refresh_token }).unwrap())
                .send();
            match res {
                Ok(res) => {
                    let body = res.text();
                    match body {
                        Ok(body) => {
                            let as_json = serde_json::from_str::<RefreshResponse>(&body);
                            match as_json {
                                Ok(tokens) => {
                                    Session::set_token(tokens);
                                    Ok(())
                                }
                                Err(e) => {
                                    println!("Error parsing json {}", e.to_string());
                                    Err(())
                                }
                            }
                        }
                        Err(err) => {
                            println!("Invalid Response from the server {}", err.to_string());
                            Router::set("auth/login");
                            Err(())
                        }
                    }
                }
                Err(e) => {
                    println!("Error occurred while attempting refresh {}", e.to_string());
                    Router::set("auth/login");
                    Err(())
                }
            }
        }
        None => {
            println!("No refresh token found navigate to login");
            Router::set("auth/login");
            Err(())
        }
    }
}

pub enum NetErr {
    Reqwest(Error),
    Refresh()
}

impl Into<String> for NetErr{
    fn into(self) -> String {
        match self {
            NetErr::Reqwest(error) => error.to_string(),
            NetErr::Refresh() => "Error occurred while refreshing tokens".into(),
        }
    }
}

impl From<Error> for NetErr{
    fn from(value: Error) -> Self {
        return NetErr::Reqwest(value);
    }
}

fn post_inner<Body>(
    path: &str,
    body: &Option<Body>,
    count: usize,
) -> Result<reqwest::blocking::Response, NetErr>
where
    Body: Serialize,
{
    let (access_token, refresh_token) = Session::get_tokens();

    let client = get_client(ClientModes::POST, &path, body, access_token);

    let res = client.send()?;
    if res.status().as_u16() == 401 {
        println!("UNAUTHORIZED ATTEMPTING REFRESH");
        if count < 3 {
            match refresh_the_token(refresh_token)
            {
                Ok(_) => Ok(post_inner(path, body, count + 1 )?),
                Err(_) => Ok(post_inner(path, body, count + 1 )?)
            }
        }else{
            println!("Max retires reached for refresh");
            Err(NetErr::Refresh())
        }
    }else{
        return Ok(res);
    }
}

pub fn post<Body>(path: &str, body: &Option<Body>) -> Result<reqwest::blocking::Response, NetErr>
where
    Body: Serialize,
{
    return post_inner(path, &body, 0);
}

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
    Session::init();
    Router::init("auth/login");
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
                let current_path = Router::current_path();
                current_path
            };
            let path_changed = { Router::path_changed() };

            let c = build_route(path, r, path_changed);
            Router::reset_path_changed();
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
