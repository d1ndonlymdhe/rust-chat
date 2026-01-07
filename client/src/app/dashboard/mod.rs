

use ui::{components::{common::{Component, Length}, layout::Layout, text_layout::TextLayout}, raylib::color::Color};

use crate::{no_op, utils::{router::Route, session::Session}};

fn dashboard() -> Component{
    let (access_token,refresh_token) = Session::get_tokens();
    let access_token = access_token.unwrap_or("No access token".into());
    let refresh_token = refresh_token.unwrap_or("No refresh token".into());

    Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::LIGHTGRAY)
        .children(vec![
            TextLayout::get_builder()
            .dim((Length::FILL,Length::FIT))
            .wrap(true)
            .content(&format!("Access Token {}", access_token))
            .build(),
            TextLayout::get_builder()
            .dim((Length::FILL,Length::FIT))
            .wrap(true)
            .content(&format!("Refresh Token {}", refresh_token))
            .build()
        ])
        .build()
}

pub fn dashboard_route()->Route {
    Route::leaf("dashboard", no_op(), no_op(), Box::new(||{dashboard()}))
}