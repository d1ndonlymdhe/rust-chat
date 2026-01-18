use std::{
    sync::{OnceLock, RwLock},
    thread,
};

use shared::routes::chat::conversation::{ConversationWithMembers, CreateConversationResponse, GetConversationResponse};

use crate::{UI_REBUILD_SIGNAL_SEND, utils::fetch::{Response, fetch}};

pub struct ConversationPageState {
    pub conversations: Vec<ConversationWithMembers>,
    pub selected_conversation_id: Option<i32>,
    pub loading: bool,
    pub error: Option<String>,
}
impl ConversationPageState {
    fn new() -> Self {
        return Self {
            conversations: vec![],
            selected_conversation_id: None,
            loading: false,
            error: None,
        };
    }
    fn set_conversations(&mut self, new_conversations: Vec<ConversationWithMembers>) {
        self.conversations = new_conversations;
    }
    fn set_selected_conversation_id(&mut self, new_id: Option<i32>) {
        self.selected_conversation_id = new_id;
    }
    fn set_loading(&mut self, new_loading: bool) {
        self.loading = new_loading;
    }
    fn set_error(&mut self, new_error: Option<String>) {
        self.error = new_error;
    }
}

static CONVERSATION_PAGE_STATE: OnceLock<RwLock<Option<ConversationPageState>>> = OnceLock::new();

pub struct ConversationsState;

impl ConversationsState {
    pub fn init() {
        match CONVERSATION_PAGE_STATE.get() {
            Some(v) => {
                let has_state = {
                    let state = v.read().unwrap();
                    state.is_some()
                };
                if !has_state {
                    let mut state = v.write().unwrap();
                    state.replace(ConversationPageState::new());
                }
            }
            None => {
                CONVERSATION_PAGE_STATE
                    .set(RwLock::new(Some(ConversationPageState::new())))
                    .ok()
                    .unwrap();
            }
        };
        load_conversations();
    }
    pub fn de_init() {
        match CONVERSATION_PAGE_STATE.get() {
            Some(v) => {
                let mut state = v.write().unwrap();
                state.take();
            }
            None => {}
        };
    }
    pub fn state() -> &'static RwLock<Option<ConversationPageState>> {
        return CONVERSATION_PAGE_STATE.get().unwrap();
    }
    pub fn conversations() -> Vec<ConversationWithMembers> {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.conversations.clone();
    }
    pub fn selected_conversation_id() -> Option<i32> {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.selected_conversation_id;
    }
    pub fn loading() -> bool {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.loading;
    }
    pub fn error() -> Option<String> {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.error.clone();
    }
    pub fn set_conversations(new_conversations: Vec<ConversationWithMembers>) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_conversations(new_conversations);
    }
    pub fn set_selected_conversation_id(new_id: Option<i32>) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_selected_conversation_id(new_id);
    }
    pub fn set_loading(new_loading: bool) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_loading(new_loading);
    }
    pub fn set_error(new_error: Option<String>) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_error(new_error);
    }
}

fn load_conversations() {
    let loading = ConversationsState::loading();
    if loading {
        return;
    }
    ConversationsState::set_loading(true);
    thread::spawn(|| {
        let resp = fetch::<()>(crate::utils::fetch::ClientModes::GET, "/chat/conversation", &None);
        let result = match resp {
            Ok(v) => v,
            Err(e) => {
                ConversationsState::set_error(Some(e.into()));
                ConversationsState::set_loading(false);
                return;
            }
        };
        let result_text = result.text().unwrap();
        println!("Fetched conversations: {}", result_text);
        let conversations_try_json = serde_json::from_str::<Response<GetConversationResponse>>(&result_text);
        match conversations_try_json {
            Ok(conversations) => {
                if !conversations.success {
                    ConversationsState::set_error(Some("Failed to fetch conversations".into()));
                }else{
                    ConversationsState::set_conversations(conversations.data.unwrap());
                }
            }
            Err(e) => {
                ConversationsState::set_error(Some(e.to_string()));
            }
        }
        ConversationsState::set_loading(false);
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    });
}
