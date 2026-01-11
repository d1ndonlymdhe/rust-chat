use std::sync::{OnceLock, RwLock};
use shared::routes::users::search::SearchUser;

pub struct SearchPageState {
    pub search_query: String,
    pub results: Vec<SearchUser>,
    pub loading: bool,
    pub error: Option<String>,
}

impl SearchPageState {
    fn new() -> Self {
        Self {
            search_query: String::new(),
            results: vec![],
            loading: false,
            error: None,
        }
    }
}

static SEARCH_PAGE_STATE: OnceLock<RwLock<Option<SearchPageState>>> = OnceLock::new();

pub struct SearchState;

impl SearchState {
    pub fn init() {
        match SEARCH_PAGE_STATE.get() {
            Some(v) => {
                let has_state = {
                    let state = v.read().unwrap();
                    state.is_some()
                };
                if !has_state {
                    let mut state = v.write().unwrap();
                    state.replace(SearchPageState::new());
                }
            }
            None => {
                SEARCH_PAGE_STATE
                    .set(RwLock::new(Some(SearchPageState::new())))
                    .ok()
                    .unwrap();
            }
        }
    }

    pub fn de_init() {
        match SEARCH_PAGE_STATE.get() {
            Some(v) => {
                let mut state = v.write().unwrap();
                state.take();
            }
            None => {}
        }
    }

    fn state() -> &'static RwLock<Option<SearchPageState>> {
        SEARCH_PAGE_STATE
            .get()
            .expect("Search Page State not initialized")
    }

    pub fn set_search_query(new_query: String) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.search_query = new_query;
    }

    pub fn set_loading(is_loading: bool) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.loading = is_loading;
    }

    pub fn set_results(new_results: Vec<SearchUser>) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.results = new_results;
    }

    pub fn set_error(new_error: Option<String>) {
        let mut state = Self::state().write().unwrap();
        let state = state.as_mut().unwrap();
        state.error = new_error;
    }

    pub fn search_query() -> String {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.search_query.clone()
    }

    pub fn results() -> Vec<SearchUser> {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        state.results.clone()
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

    pub fn read_state() -> SearchPageState {
        let state = Self::state().read().unwrap();
        let state = state.as_ref().unwrap();
        SearchPageState {
            search_query: state.search_query.clone(),
            results: state.results.clone(),
            loading: state.loading,
            error: state.error.clone(),
        }
    }
}
