use std::sync::{OnceLock, RwLock};

use shared::routes::auth::refresh::RefreshResponse;

struct SessionT {
    access_token: Option<String>,
    refresh_token: Option<String>,
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
            }))
            .ok()
            .expect("Session already initialized");
    }

    pub fn get_tokens() -> (Option<String>, Option<String>) {
        let session = Self::session().read().unwrap();
        (session.access_token.clone(), session.refresh_token.clone())
    }

    pub fn set_access(token: Option<String>) {
        Self::session().write().unwrap().access_token = token;
    }

    pub fn set_refresh(token: Option<String>) {
        Self::session().write().unwrap().refresh_token = token;
    }

    pub fn set_token(tokens: RefreshResponse) {
        let mut session = Self::session().write().unwrap();
        session.access_token = Some(tokens.access_token);
        session.refresh_token = Some(tokens.refresh_token);
    }
}
