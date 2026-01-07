use ui::{components::{common::{Alignment, Component, Length}, layout::Layout}, raylib::color::Color};

use crate::{app::{auth::auth_route, dashboard::dashboard_route}, no_op, utils::router::{Route, outlet}};

mod auth;
mod dashboard;
fn app() -> Component {
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

pub fn app_route() -> Route {
    return Route::container(
        "root",
        no_op(),
        no_op(),
        "root_outlet",
Box::new(||app()),
vec![auth_route(),dashboard_route()]
    )
}
