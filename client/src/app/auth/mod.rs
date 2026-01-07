use ui::components::{
    common::{Alignment, Component, Length},
    layout::Layout,
};

use crate::{
    app::auth::{login::login_route, signup::signup_route},
    no_op,
    utils::router::{Route, outlet},
};

mod login;
mod signup;
mod signup_store;
mod login_store;
fn auth_screen() -> Component {
    Layout::get_row_builder()
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(60), Length::FILL))
                .children(vec![outlet("auth_outlet")])
                .build(),
        ])
        .build()
}

pub fn auth_route() -> Route {
    return Route::container(
        "auth",
        no_op(),
        no_op(),
        "auth_outlet",
        Box::new(|| auth_screen()),
        vec![login_route(), signup_route()],
    );
}
