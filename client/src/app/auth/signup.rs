use shared::{
    ResponseStruct,
    routes::auth::signup::{SignupRequest, SignupResponse},
};
use std::thread;
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{
    UI_REBUILD_SIGNAL_SEND,
    app::auth::signup_store::{SignupPageState, SignupState},
    no_op,
    utils::{
        fetch::{ClientModes, fetch},
        popup::popup,
        router::{Route, Router},
        state::as_state,
        text_input::{TextInputType, text_input},
    },
};

fn execute_signup() {
    SignupState::set_loading(true);
    let username = SignupState::username();
    let password = SignupState::password();
    thread::spawn(|| {
        let req_body = SignupRequest {
            email: username.into(),
            password: password.into(),
        };
        let res = fetch(ClientModes::POST, "/auth/signup", &Some(req_body));
        match res {
            Ok(res) => {
                let body = res.text().unwrap();
                println!("Signup response body: {}", body);
                let res = serde_json::from_str::<ResponseStruct<SignupResponse>>(&body).unwrap();
                if res.success {
                    Router::push("auth/login");
                } else {
                    SignupState::set_error(Some(res.message));
                    SignupState::set_loading(false);
                }
            }
            Err(e) => {
                SignupState::set_loading(false);
                SignupState::set_error(Some(e.into()));
            }
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    });
}

fn signup_page() -> Component {
    let SignupPageState {
        username,
        password,
        loading,
        error,
    } = SignupState::read_state();

    let email_box = {
        let username = username;
        text_input(
            username,
            as_state(move |new_email| SignupState::set_username(new_email.into())),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let password = password;
        text_input(
            password,
            as_state(move |new_password| {
                SignupState::set_password(new_password.into());
            }),
            TextInputType::Password,
        )
    };

    let mut form_children = vec![
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
                execute_signup();
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
                Router::push("auth/login");
                false
            }))
            .build(),
    ];

    if loading {
        form_children.push(
            TextLayout::get_builder()
                .content("Loading...")
                .dim((Length::FILL, Length::FIT))
                .build(),
        );
    }

    let mut children: Vec<Component> = vec![
            TextLayout::get_builder()
                .content("Signup")
                .font_size(40)
                .build(),
            Layout::get_col_builder()
                .gap(10)
                .cross_align(Alignment::Center)
                .children(form_children)
                .build(),
    ];
    if let Some(message) = error {
        children.push(popup(&message, Box::new(|| {
            SignupState::set_error(None);
        })));
    }

    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::RED)
        .flex(9.5)
        .cross_align(Alignment::Center)
        .padding((10, 10, 10, 10))
        .gap(30)
        .children(children)
        .build();
}

pub fn signup_route() -> Route {
    return Route::leaf(
        "signup",
        Box::new(|| {
            SignupState::init();
        }),
        Box::new(|| {
            SignupState::de_init();
        }),
        Box::new(|| signup_page()),
    );
}
