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
fn main() {
    UIRoot::start(Box::new(|| ui_builder()), (1920, 1000), "Hello from lib");
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
