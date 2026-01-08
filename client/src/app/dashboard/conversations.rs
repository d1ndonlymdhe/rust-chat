use ui::{components::{common::Component, layout::Layout, text_layout::TextLayout}, raylib::color::Color};

use crate::{no_op, utils::router::Route};

fn conversation_layout() -> Component {
    Layout::get_col_builder()
        .children(vec![
            TextLayout::get_builder()
                .content("Conversations Page")
                .text_color(Color::WHITE)
                .font_size(32)
                .build(),
            // Additional conversation components can be added here
        ])
        .build()
}

pub fn conversations_route() -> Route {
    Route::leaf(
        "conversations",
        no_op(),
        no_op(),
        Box::new(|| conversation_layout()),
    )
}