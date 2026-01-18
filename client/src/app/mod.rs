use ui::{
    components::{
        common::{Component},
        layout::Layout,
    },
};

use crate::{
    app::{auth::auth_route, dashboard::dashboard_route},
    no_op,
    utils::router::{Route, outlet},
};

mod auth;
mod dashboard;
fn app() -> Component {
    Layout::get_col_builder()
        .children(vec![outlet("root_outlet")])
        .build()
}

pub fn app_route() -> Route {
    return Route::container(
        "root",
        no_op(),
        no_op(),
        "root_outlet",
        Box::new(|params| app()),
        vec![auth_route(), dashboard_route()],
    );
}
