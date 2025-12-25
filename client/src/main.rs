use ui::{
    components::{common::Component, layout::Layout, root::UIRoot},
    raylib::color::Color,
};

use crate::auth::{auth_screen};
mod auth;
extern crate ui;
fn main() {
    UIRoot::start(Box::new(|| ui_builder()), (1920, 1000), "Hello from lib");
}

fn ui_builder() -> Component {
    let auth_screen = auth_screen();

    return Layout::get_col_builder()
        .bg_color(Color::WHITE)
        .children(vec![
            auth_screen
        ])
        .build();
}
