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
    Router::init("root");
    // Router::init("auth/signup");
    UIRoot::start(
        Box::new(move || {
            let r = app_route();
            let (path,_,path_changed) = {
                let current_path = Router::current_path(true);
                current_path
            };
            let c = build_route(path, r, path_changed);
            // Router::reset_path_changed();
            c
        }),
        (1920, 1000),
        "Raylib Rocket Chat Client",
        ui_rebuild_signal_recv,
    );
}


