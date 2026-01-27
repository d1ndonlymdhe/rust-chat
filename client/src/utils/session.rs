use std::sync::{OnceLock, RwLock};

use shared::routes::auth::refresh::RefreshResponse;

struct SessionT {
    access_token: Option<String>,
    refresh_token: Option<String>,
    user_id: Option<i32>,
    username: Option<String>,
    self_loading: bool,
    self_loaded: bool,
}

static SESSION: OnceLock<RwLock<SessionT>> = OnceLock::new();

/// Thread-safe session store helper.
pub struct Session;

impl Session {
    fn session() -> &'static RwLock<SessionT> {
        SESSION.get().expect("Session not initialized")
    }

    pub fn init() {
        SESSION
            .set(RwLock::new(SessionT {
                access_token: None,
                refresh_token: None,
                user_id: None,
                username: None,
                self_loaded: false,
                self_loading: false,
            }))
            .ok()
            .expect("Session already initialized");
    }

    pub fn get_tokens() -> (Option<String>, Option<String>) {
        let session = Self::session().read().unwrap();
        (session.access_token.clone(), session.refresh_token.clone())
    }

    // pub fn set_access(token: Option<String>) {
    //     Self::session().write().unwrap().access_token = token;
    // }

    // pub fn set_refresh(token: Option<String>) {
    //     Self::session().write().unwrap().refresh_token = token;
    // }

    pub fn set_token(tokens: RefreshResponse) {
        let mut session = Self::session().write().unwrap();
        session.access_token = Some(tokens.access_token);
        session.refresh_token = Some(tokens.refresh_token);
    }

    pub fn set_self_details(user_id: i32, username: String) {
        let mut session = Self::session().write().unwrap();
        session.user_id = Some(user_id);
        session.username = Some(username);
    }

    pub fn get_self_details() -> (Option<i32>, Option<String>) {
        let session = Self::session().read().unwrap();
        (session.user_id, session.username.clone())
    }

    pub fn get_username() -> Option<String> {
        let session = Self::session().read().unwrap();
        session.username.clone()
    }
    pub fn get_user_id() -> Option<i32> {
        let session = Self::session().read().unwrap();
        session.user_id
    }

    pub fn self_loading() -> bool {
        let session = Self::session().read().unwrap();
        session.self_loading
    }
    pub fn self_loaded() -> bool {
        let session = Self::session().read().unwrap();
        session.self_loaded
    }
    pub fn set_self_loading(loading: bool) {
        let mut session = Self::session().write().unwrap();
        session.self_loading = loading;
    }
    pub fn set_self_loaded(loaded: bool) {
        let mut session = Self::session().write().unwrap();
        session.self_loaded = loaded;
    }

}
