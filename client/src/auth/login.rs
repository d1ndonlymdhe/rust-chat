use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use ui::{
    components::{
        common::{Component, Length, def_key_handler},
        layout::Layout,
        text_input::TextInput,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

struct AuthState {
    token: Option<String>,
    screen: AuthScreen,
}
impl AuthState {
    fn new() -> Self {
        Self {
            token: None,
            screen: AuthScreen::Login("".to_string(), "".to_string()),
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
fn as_state<T>(v: T) -> State<T> {
    return Rc::new(RefCell::new(v));
}
#[derive(Clone)]
enum AuthScreen {
    Login(String, String),
    Signup(String, String),
}

lazy_static! {
    static ref AUTH_STATE: Arc<Mutex<AuthState>> = Arc::new(Mutex::new(AuthState::new()));
}

pub fn auth_screen() -> Component {
    let active_screen = {
        let state = AUTH_STATE.lock().unwrap();
        state.screen.clone()
    };
    let active_screen = match active_screen {
        AuthScreen::Login(_, _) => login_component(),
        AuthScreen::Signup(_, _) => signup_component(),
    };
    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .children(vec![
            active_screen,
            TextLayout::get_builder()
                .content("SWITCH")
                .dim((Length::FILL,Length::FILL))
                .bg_color(Color::WHEAT)
                .dbg_name("SWITCH")
                .flex(0.5)
                .on_click(Box::new(|_| {
                    let mut state = AUTH_STATE.lock().unwrap();
                    println!("SWITCH LOCK OBTAINED");
                    state.toggle_active_screen();
                    false
                }))
                .build(),
        ])
        .bg_color(Color::LIME)
        .build();
}

fn login_component() -> Component {
    let email_box = {
        let login_params = {
            let state = AUTH_STATE.lock().unwrap();
            println!("EMAIL STATE LOCK OBTAINED");
            state.get_login_params()
        };
        text_input(
            login_params.0,
            as_state(move |new_email| {
                let mut state = AUTH_STATE.lock().unwrap();
                println!("EMAIL CHANGE STATE LOCK OBTAINED");
                state.set_login_state(new_email, &login_params.1);
            }),
        )
    };
    let pass_box = {
        let login_params = {
            let state = AUTH_STATE.lock().unwrap();
            println!("PASS STATE LOCK OBTAINED");
            state.get_login_params()
        };
        text_input(
            login_params.1,
            as_state(move |new_pass| {
                let mut state = AUTH_STATE.lock().unwrap();
                println!("PASS CHANGE STATE LOCK OBTAINED");
                state.set_login_state(&login_params.0, new_pass);
            }),
        )
    };
    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::RED)
        .flex(9.5)
        .children(vec![
            TextLayout::get_builder()
                .content("Login")
                .font_size(40)
                .build(),
            TextLayout::get_builder().content("Email").build(),
            email_box,
            TextLayout::get_builder().content("Password").build(),
            pass_box,
        ])
        .build();
}

fn signup_component() -> Component {
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
        )
    };
    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::RED)
        .flex(9.5)
        .gap(10)
        .children(vec![
            TextLayout::get_builder()
                .content("Signup")
                .font_size(40)
                .build(),
            TextLayout::get_builder().content("Email").build(),
            email_box,
            TextLayout::get_builder().content("Password").build(),
            pass_box,
        ])
        .build();
}

fn text_input(value: String, set_val: State<dyn FnMut(&str) -> ()>) -> Component {
    return TextInput::get_builder()
        .content(&value.clone())
        .dim((Length::FILL, Length::FIT))
        .on_key(Box::new(move |ev| {
            let (_, new_email) = def_key_handler(ev, &value);
            set_val.clone().borrow_mut()(&new_email);
            false
        }))
        .build();
}
