use std::sync::{
    OnceLock,
    mpsc::{self, Receiver, Sender},
};
use ui::components::root::UIRoot;

use crate::{
    app::app_route,
    utils::{
        router::{Router, build_route},
        session::Session,
    },
};

mod app;
mod utils;

extern crate ui;

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
            let r = app_route();
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
