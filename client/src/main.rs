use ui::{
    components::{common::Component, layout::Layout, root::UIRoot, text_layout::TextLayout},
    raylib::color::Color,
};

extern crate ui;
fn main() {
    UIRoot::start(Box::new(|| ui_builder()), (1920, 1000), "Hello from lib");
}

fn ui_builder() -> Component {
    return Layout::get_col_builder()
        .bg_color(Color::WHITE)
        .children(vec![
            TextLayout::get_builder().content("Hello World").build(),
        ])
        .build();
}
