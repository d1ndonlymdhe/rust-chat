use std::sync::{OnceLock, RwLock};

pub struct SignupPageState {
    pub username: String,
    pub password: String,
    pub loading: bool,
    pub error: Option<String>,
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

static SIGNUP_PAGE_STATE: OnceLock<RwLock<Option<SignupPageState>>> = OnceLock::new();

pub struct SignupState;

impl SignupState {
    pub fn init() {
        println!("Initializing signup state");
        match SIGNUP_PAGE_STATE.get() {
            Some(v) => {
                let has_state = {
                    let state = v.read().unwrap();
                    state.is_some()
                };
                if !has_state {
                    let mut state = v.write().unwrap();
                    state.replace(SignupPageState::new());
                }
            }
            None => {
                SIGNUP_PAGE_STATE
                    .set(RwLock::new(Some(SignupPageState::new())))
                    .ok()
                    .unwrap();
            }
        };
    }
    pub fn de_init() {
        match SIGNUP_PAGE_STATE.get() {
            Some(v) => {
                let mut state = v.write().unwrap();
                state.take();
            }
            None => {}
        }
    }
    fn state() -> &'static RwLock<Option<SignupPageState>> {
        return SIGNUP_PAGE_STATE
            .get()
            .expect("Signup state not initialized");
    }
    pub fn set_password(new_password: String) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.set_password(new_password);
    }
    pub fn set_username(new_username: String) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.set_username(new_username);
    }
    pub fn set_loading(new_loading: bool) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.set_loading(new_loading);
    }
    pub fn set_error(new_error: Option<String>) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.set_error(new_error);
    }
    pub fn username() -> String {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.username.clone()
    }
    pub fn password() -> String {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.password.clone()
    }
    pub fn loading() -> bool {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.loading
    }
    pub fn error() -> Option<String> {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.error.clone()
    }
    pub fn read_state() -> SignupPageState {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        SignupPageState {
            username: state.username.clone(),
            password: state.password.clone(),
            loading: state.loading,
            error: state.error.clone(),
        }
    }
}
