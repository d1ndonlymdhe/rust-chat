use std::{
    thread,
};
use shared::{ResponseStruct, routes::auth::{login::{LoginRequest, LoginResponse}, refresh::RefreshResponse}};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::{UI_REBUILD_SIGNAL_SEND, app::auth::login_store::{LoginPageState, LoginState}, utils::{
    fetch::{ClientModes, fetch}, popup::popup, router::{Route, Router}, session::Session, state::as_state, text_input::{TextInputType, text_input}
}};


fn execute_login() {
    LoginState::set_loading(true);
    let username = LoginState::username();
    let password = LoginState::password();
    thread::spawn(|| {
        let req_body = LoginRequest {
            email: username.into(),
            password: password.into(),
        };
        let res = fetch(ClientModes::POST, "/auth/login", &Some(req_body));
        match res {
            Ok(body) => {
                let body_text = body.text().unwrap();
                let body_data = serde_json::from_str::<ResponseStruct<LoginResponse>>(&body_text).unwrap();
                if body_data.success {
                    let LoginResponse {access_token,refresh_token} = body_data.data.unwrap();
                    Session::set_token(RefreshResponse{
                        access_token,
                        refresh_token
                    });
                    Router::push("dashboard/conversations");
                }else{
                    let message = body_data.message;
                    LoginState::set_error(Some(message));
                    LoginState::set_loading(false);
                }
            }
            Err(e) => {
                LoginState::set_loading(false);
                LoginState::set_error(Some(e.into()));
            }
        }
        LoginState::set_loading(false);
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    });
}

fn login_page() -> Component {
    let LoginPageState{username,password,loading,error} = LoginState::read_state();

    let email_box = {
        let username = username;
        text_input(
            username,
            as_state(move |new_email| {
                LoginState::set_username(new_email.into())
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let password = password;
        text_input(
            password,
            as_state(move |new_password| {
                LoginState::set_password(new_password.into());
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
                execute_login();
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

    if loading {
        form_children.push(
            TextLayout::get_builder()
            .content("Loading...")
            .dim((Length::FILL, Length::FIT))
            .build());
    }

    let mut children: Vec<Component> = vec![
        TextLayout::get_builder()
            .content("Login")
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
            LoginState::set_error(None);
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

pub fn login_route() -> Route {
    return Route::leaf(
        "login",
        Box::new(|| {
            LoginState::init();
        }),
        Box::new(|| {
            LoginState::de_init();
        }),
        Box::new(|| login_page()),
    );
}
