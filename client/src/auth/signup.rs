use std::sync::Mutex;

use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    ROUTER,
    utils::{
        router::Route,
        state::as_state,
        text_input::{TextInputType, text_input},
    },
};

struct SignupPageState {
    username: String,
    password: String,
    loading: bool,
    error: Option<String>,
}

impl SignupPageState {
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

static SIGNUP_PAGE_STATE: Mutex<Option<SignupPageState>> = Mutex::new(None);

fn login_page() -> Component {
    let email_box = {
        let username = {
            let state = SIGNUP_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.username.clone()
        };
        text_input(
            username,
            as_state(move |new_email| {
                let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_username(new_email.into())
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let password = {
            let state = SIGNUP_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.password.clone()
        };
        text_input(
            password,
            as_state(move |new_email| {
                let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
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
            .content("Login Instead")
            .dbg_name("SwitchSignup")
            .on_click(Box::new(move |_| {
                let mut router = ROUTER.get().unwrap().lock().unwrap();
                router.push("auth/login");
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
                .content("Signup")
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

pub fn signup_route() -> Route {
    return Route::leaf(
        "signup",
        Box::new(|| {
            let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
            state.replace(SignupPageState::new());
        }),
        Box::new(|| {
            let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
            state.take();
        }),
        Box::new(|| login_page()),
    );
}
