use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use lazy_static::lazy_static;
use shared::routes::auth::signup::{SignupRequest, SignupResponse};
use ui::{
    components::{
        common::{Alignment, Component, Length, def_key_handler},
        layout::Layout,
        text_input::TextInput,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::UI_REBUILD_SIGNAL_SEND;

pub struct AuthState {
    pub screen: AuthScreen,
    pub error: Option<String>,
    pub loading: bool,
}

fn execute_signup() {
    let (email, password) = {
        let mut state = AUTH_STATE.lock().unwrap();
        state.loading = true;
        state.error = None;
        state.get_signup_params()
    };
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let req_body = serde_json::to_string(&(SignupRequest { email, password })).unwrap();
        let res = client
            .post("http://localhost:8000/auth/signup")
            .body(req_body)
            .send();
        let mut state = AUTH_STATE.lock().unwrap();
        match res {
            Ok(v) => {
                println!("{} {}", v.status(), v.text().unwrap());
                state.loading = false;
                UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
            }
            Err(e) => {
                print!("Net Err: {:#?}", e);
                state.loading = false;
                state.error = Some(e.to_string());
                UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
            }
        }
    });
}

fn execute_login() {
    let (email, password) = {
        let mut state = AUTH_STATE.lock().unwrap();
        state.loading = true;
        state.error = None;
        state.get_login_params()
    };
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let req_body = serde_json::to_string(&(SignupRequest { email, password })).unwrap();
        let res = client
            .post("http://localhost:8000/auth/login")
            .body(req_body)
            .send();
        let mut state = AUTH_STATE.lock().unwrap();
        match res {
            Ok(v) => {
                // println!("{} {}", v.status(), v.text().unwrap());
                let o: SignupResponse = serde_json::from_str(&v.text().unwrap()).unwrap();
                state.loading = false;
                UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
            }
            Err(e) => {
                print!("Net Err: {:#?}", e);
                state.loading = false;
                state.error = Some(e.to_string());
                UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
            }
        }
    });
}

impl AuthState {
    fn new() -> Self {
        Self {
            screen: AuthScreen::Login("".to_string(), "".to_string()),
            loading: false,
            error: None,
        }
    }
    fn get_login_params(&self) -> (String, String) {
        match &self.screen {
            AuthScreen::Login(email, password) => (email.clone(), password.clone()),
            AuthScreen::Signup(_, _) => {
                panic!("Called get login params on signup state")
            }
        }
    }
    fn get_signup_params(&self) -> (String, String) {
        match &self.screen {
            AuthScreen::Signup(email, password) => (email.clone(), password.clone()),
            AuthScreen::Login(_, _) => {
                panic!("Called get signup params on login state")
            }
        }
    }
    fn set_login_state(&mut self, email: &str, password: &str) {
        self.screen = AuthScreen::Login(email.into(), password.into())
    }
    fn set_signup_state(&mut self, email: &str, password: &str) {
        self.screen = AuthScreen::Signup(email.into(), password.into())
    }
    fn toggle_active_screen(&mut self) {
        match self.screen {
            AuthScreen::Login(_, _) => self.set_signup_state("", ""),
            AuthScreen::Signup(_, _) => {
                self.set_login_state("", "");
            }
        }
    }
}
type State<T> = Rc<RefCell<T>>;
pub fn as_state<T>(v: T) -> State<T> {
    return Rc::new(RefCell::new(v));
}
#[derive(Clone)]
pub enum AuthScreen {
    Login(String, String),
    Signup(String, String),
}

lazy_static! {
    pub static ref AUTH_STATE: Arc<Mutex<AuthState>> = Arc::new(Mutex::new(AuthState::new()));
}

pub fn auth_screen() -> Component {
    let (active_screen, loading, error) = {
        let state = AUTH_STATE.lock().unwrap();
        (state.screen.clone(), state.loading, state.error.clone())
    };
    let active_screen = match active_screen {
        AuthScreen::Login(_, _) => login_component(loading, error.clone()),
        AuthScreen::Signup(_, _) => signup_component(loading, error),
    };
    return Layout::get_row_builder()
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(60), Length::FILL))
                .children(vec![active_screen])
                .build(),
        ])
        .build();
}

fn login_component(loading: bool, error: Option<String>) -> Component {
    let email_box = {
        let login_params = {
            let state = AUTH_STATE.lock().unwrap();
            state.get_login_params()
        };
        text_input(
            login_params.0,
            as_state(move |new_email| {
                let mut state = AUTH_STATE.lock().unwrap();
                state.set_login_state(new_email, &login_params.1);
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let login_params = {
            let state = AUTH_STATE.lock().unwrap();
            state.get_login_params()
        };
        text_input(
            login_params.1,
            as_state(move |new_pass| {
                let mut state = AUTH_STATE.lock().unwrap();
                state.set_login_state(&login_params.0, new_pass);
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
            .dbg_name("SwitchSignup")
            .on_click(Box::new(move |_| {
                let mut state = AUTH_STATE.lock().unwrap();
                if !loading {
                    state.toggle_active_screen();
                }
                false
            }))
            .build(),
    ];
    if let Some(err) = error.clone() {
        let err_msg = format!("Error: {}", err);
        form_children.push(TextLayout::get_builder().content(&err_msg).build());
    }
    if loading {
        form_children.push(
            TextLayout::get_builder()
                .content(if loading { "LOADING" } else { "NOT LOADING" })
                .build(),
        )
    }
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

fn signup_component(loading: bool, error: Option<String>) -> Component {
    let email_box = {
        let signup_params = {
            let state = AUTH_STATE.lock().unwrap();
            state.get_signup_params()
        };
        text_input(
            signup_params.0,
            as_state(move |new_email| {
                let mut state = AUTH_STATE.lock().unwrap();
                state.set_signup_state(new_email, &signup_params.1);
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let signup_params = {
            let state = AUTH_STATE.lock().unwrap();
            state.get_signup_params()
        };
        text_input(
            signup_params.1,
            as_state(move |new_pass| {
                let mut state = AUTH_STATE.lock().unwrap();
                state.set_signup_state(&signup_params.0, new_pass);
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
            .dbg_name("SwitchLogin")
            .on_click(Box::new(move |_| {
                let mut state = AUTH_STATE.lock().unwrap();
                if !loading {
                    state.toggle_active_screen();
                }
                false
            }))
            .build(),
    ];
    if let Some(err) = error.clone() {
        let err_msg = format!("Error: {}", err);
        form_children.push(TextLayout::get_builder().content(&err_msg).build());
    }
    if loading {
        form_children.push(
            TextLayout::get_builder()
                .content(if loading { "LOADING" } else { "NOT LOADING" })
                .build(),
        );
    }
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

pub enum TextInputType {
    Password,
    Text,
}

pub fn text_input(
    value: String,
    set_val: State<dyn FnMut(&str) -> ()>,
    input_type: TextInputType,
) -> Component {
    let content = match input_type {
        TextInputType::Password => {
            let hidden_str = "*".repeat(value.len());
            &(hidden_str.clone())
        }
        TextInputType::Text => &(value.clone()),
    };

    return TextInput::get_builder()
        .content(content)
        .main_align(Alignment::Center)
        .dim((Length::FILL, Length::FitPer(180)))
        .font_size(26)
        .padding((5, 0, 5, 0))
        .wrap(true)
        .on_key(Box::new(move |ev| {
            let (_, new_email) = def_key_handler(ev, &value);
            set_val.clone().borrow_mut()(&new_email);
            false
        }))
        .build();
}
