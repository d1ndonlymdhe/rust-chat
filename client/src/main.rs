use std::sync::{OnceLock, mpsc::{self, Receiver, Sender}};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        root::UIRoot,
    },
    raylib::color::Color,
};

use crate::auth::auth_screen;
mod auth;
extern crate ui;

pub static UI_REBUILD_SIGNAL_SEND: OnceLock<Sender<()>> = OnceLock::new();

fn init_channel() -> Receiver<()> {
    let (tx, rx) = mpsc::channel();
    UI_REBUILD_SIGNAL_SEND.set(tx).ok().unwrap();
    rx
}
fn main() {
    let UI_REBUILD_SIGNAL_RECV = init_channel();
    UIRoot::start(Box::new(|| ui_builder()), (1920, 1000), "Hello from lib",UI_REBUILD_SIGNAL_RECV);
}

fn ui_builder() -> Component {
    let auth_screen = auth_screen();

    return Layout::get_row_builder()
        .bg_color(Color::WHEAT)
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(50), Length::FILL))
                .children(vec![auth_screen])
                .build(),
        ])
        .build();
}
