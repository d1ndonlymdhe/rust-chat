use std::sync::Mutex;

use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::utils::{
    router::{Route, Router},
    state::as_state,
    text_input::{TextInputType, text_input},
};

struct LoginPageState {
    username: String,
    password: String,
    loading: bool,
    error: Option<String>,
}

impl LoginPageState {
    fn new() -> Self {
        return Self {
            username: "".into(),
            password: "".into(),
            loading: false,
            error: None,
        };
    }
    fn set_password(&mut self, new_password: String) {
        self.password = new_password;
    }
    fn set_username(&mut self, new_username: String) {
        self.username = new_username;
    }
    fn set_loading(&mut self, new_loading: bool) {
        self.loading = new_loading;
    }
    fn set_error(&mut self, new_error: Option<String>) {
        self.error = new_error;
    }
}

static LOGIN_PAGE_STATE: Mutex<Option<LoginPageState>> = Mutex::new(None);

fn login_page() -> Component {
    let email_box = {
        let username = {
            let state = LOGIN_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.username.clone()
        };
        text_input(
            username,
            as_state(move |new_email| {
                let mut state = LOGIN_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_username(new_email.into())
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let password = {
            let state = LOGIN_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.password.clone()
        };
        text_input(
            password,
            as_state(move |new_email| {
                let mut state = LOGIN_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_password(new_email.into())
            }),
            TextInputType::Password,
        )
    };
    let form_children = vec![
        TextLayout::get_builder()
            .dim((Length::FILL, Length::FIT))
            .content("Email: ")
            .build(),
        email_box,
        TextLayout::get_builder()
            .dim((Length::FILL, Length::FIT))
            .content("Password: ")
            .build(),
        pass_box,
        TextLayout::get_builder()
            .padding((5, 5, 5, 5))
            .content("Continue")
            .on_click(Box::new(|_| {
                // execute_login();
                false
            }))
            .bg_color(Color::BEIGE)
            .build(),
        TextLayout::get_builder()
            .padding((5, 5, 5, 5))
            .bg_color(Color::BEIGE)
            .dim((Length::FIT, Length::FIT))
            .wrap(false)
            .content("Signup Instead")
            .dbg_name("SwitchLogin")
            .on_click(Box::new(move |_| {
                Router::push("auth/signup");
                false
            }))
            .build(),
    ];
    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::RED)
        .flex(9.5)
        .cross_align(Alignment::Center)
        .padding((10, 10, 10, 10))
        .gap(30)
        .children(vec![
            TextLayout::get_builder()
                .content("Login")
                .font_size(40)
                .build(),
            Layout::get_col_builder()
                .gap(10)
                .cross_align(Alignment::Center)
                .children(form_children)
                .build(),
        ])
        .build();
}

pub fn login_route() -> Route {
    return Route::leaf(
        "login",
        Box::new(|| {
            let mut state = LOGIN_PAGE_STATE.lock().unwrap();
            state.replace(LoginPageState::new());
        }),
        Box::new(|| {
            let mut state = LOGIN_PAGE_STATE.lock().unwrap();
            state.take();
        }),
        Box::new(|| login_page()),
    );
}
