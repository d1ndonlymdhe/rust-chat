use ui::{
    components::{
        common::{Alignment, Component, Length, Position},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

pub fn popup(message: &str, close: Box<dyn Fn()>) -> Component {
    Layout::get_col_builder()
        .set_position(Position::Abs(0, 0))
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FILL, Length::FitPer(150)))
                .bg_color(Color {
                    r: 100,
                    g: 100,
                    b: 100,
                    a: 50,
                })
                .main_align(Alignment::Center)
                .cross_align(Alignment::Center)
                .gap(10)
                .children(vec![
                    TextLayout::get_builder()
                        .content(message)
                        .cross_align(Alignment::Center)
                        .dim((Length::FILL, Length::FIT))
                        .build(),
                    TextLayout::get_builder()
                        .bg_color(Color::BEIGE)
                        .dim((Length::FIT, Length::FIT))
                        .wrap(false)
                        .padding((5, 5, 5, 5))
                        .content("Close")
                        .dbg_name("ClosePopup")
                        .on_click(Box::new(move |_| {
                            close();
                            false
                        }))
                        .build(),
                ])
                .build(),
        ])
        .build()
}
