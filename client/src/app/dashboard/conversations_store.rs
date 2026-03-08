use std::{
    sync::{OnceLock, RwLock},
    thread,
};

use shared::routes::chat::{
    conversation::{ConversationWithMembers, GetConversationResponse},
    message::{ChatMessage, GetMessagesForConversationResponse, Message},
};

use crate::{
    UI_REBUILD_SIGNAL_SEND,
    app::dashboard::DashboardState,
    utils::fetch::{ClientModes, Response, fetch},
};

#[derive(Clone)]
pub struct ClientMessage {
    pub id: i32,
    pub sender_user_id: i32,
    pub message_type: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ChatMessage> for ClientMessage {
    fn from(value: ChatMessage) -> Self {
        return Self {
            id: value.msg_id,
            sender_user_id: value.sender_user_id,
            message_type: value.message_type,
            content: value.content,
            created_at: value.created_at,
        };
    }
}

impl From<Message> for ClientMessage {
    fn from(msg: Message) -> Self {
        ClientMessage {
            id: msg.id,
            sender_user_id: msg.sender_user_id,
            message_type: msg.message_type,
            content: msg.content,
            created_at: msg.created_at,
        }
    }
}

#[derive(Clone)]
pub struct ClientConversation {
    pub conversation: ConversationWithMembers,
    pub messages: Vec<ClientMessage>,
    pub messages_loaded: bool,
    pub messages_loading: bool,
    pub error: Option<String>,
}
impl From<ConversationWithMembers> for ClientConversation {
    fn from(conv: ConversationWithMembers) -> Self {
        ClientConversation {
            conversation: conv,
            messages: vec![],
            messages_loaded: false,
            messages_loading: false,
            error: None,
        }
    }
}

pub struct ConversationPageState {
    pub conversations: Vec<ClientConversation>,
    pub selected_conversation_id: Option<i32>,
    pub loading: bool,
    pub error: Option<String>,
    // This is used to store the state of the message input box, one for all conversations reset when switching
    pub message_draft: String,
}
impl ConversationPageState {
    fn new() -> Self {
        return Self {
            conversations: vec![],
            selected_conversation_id: None,
            loading: false,
            message_draft: String::new(),
            error: None,
        };
    }
    fn set_conversations(&mut self, new_conversations: Vec<ConversationWithMembers>) {
        self.conversations = new_conversations.into_iter().map(|c| c.into()).collect();
    }
    fn set_selected_conversation_id(&mut self, new_id: Option<i32>) {
        if self.selected_conversation_id != new_id {
            self.message_draft.clear();
        }
        self.selected_conversation_id = new_id;
    }
    fn set_loading(&mut self, new_loading: bool) {
        self.loading = new_loading;
    }
    fn set_error(&mut self, new_error: Option<String>) {
        self.error = new_error;
    }
    fn set_messages(&mut self, conversation_id: i32, messages: Vec<ClientMessage>) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            conv.messages = messages;
            conv.messages_loaded = true;
            conv.messages_loading = false;
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn add_message(&mut self, conversation_id: i32, message: ClientMessage) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            if conv
                .messages
                .iter()
                .find(|el| el.id == message.id)
                .is_none()
            {
                conv.messages.push(message);
            }
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn set_messages_loading(&mut self, conversation_id: i32, loading: bool) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            conv.messages_loading = loading;
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn set_messages_loaded(&mut self, conversation_id: i32, loaded: bool) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            conv.messages_loaded = loaded;
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn clear_messages(&mut self, conversation_id: i32) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            conv.messages.clear();
            conv.messages_loaded = false;
            conv.messages_loading = false;
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn clear_messages_error(&mut self, conversation_id: i32) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            conv.error = None;
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn set_messages_error(&mut self, conversation_id: i32, error: String) {
        if let Some(conv) = self
            .conversations
            .iter_mut()
            .find(|c| c.conversation.id == conversation_id)
        {
            conv.error = Some(error);
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    }
    fn message_draft(&self) -> String {
        self.message_draft.clone()
    }
    fn set_message_draft(&mut self, draft: String) {
        self.message_draft = draft;
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
                    load_conversations();
                }
            }
            None => {
                CONVERSATION_PAGE_STATE
                    .set(RwLock::new(Some(ConversationPageState::new())))
                    .ok()
                    .unwrap();
                load_conversations();
            }
        };
        let menu = DashboardState::menu();
        match menu {
            super::Menu::Conversations { conversation_id } => match conversation_id {
                Some(conversation_id) => {
                    if let Ok(id) = conversation_id.parse::<i32>() {
                        ConversationsState::set_selected_conversation_id(Some(id));
                    }
                }
                None => {
                    ConversationsState::set_selected_conversation_id(None);
                }
            },
            super::Menu::Search => panic!("Opened Conversation Panel while in search state"),
        }
        // start_polling();
    }
    // pub fn de_init() {
    //     match CONVERSATION_PAGE_STATE.get() {
    //         Some(v) => {
    //             let mut state = v.write().unwrap();
    //             state.take();
    //         }
    //         None => {}
    //     };
    // }
    pub fn state() -> &'static RwLock<Option<ConversationPageState>> {
        return CONVERSATION_PAGE_STATE.get().unwrap();
    }
    pub fn conversations() -> Vec<ClientConversation> {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        // TODO remove clone with lifetimes, everything seems to be 'static anyway
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
    pub fn messages_loading(conversation_id: i32) -> bool {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        if let Some(conv) = s
            .conversations
            .iter()
            .find(|c| c.conversation.id == conversation_id)
        {
            return conv.messages_loading;
        }
        false
    }
    pub fn messages_loaded(conversation_id: i32) -> bool {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        if let Some(conv) = s
            .conversations
            .iter()
            .find(|c| c.conversation.id == conversation_id)
        {
            return conv.messages_loaded;
        }
        false
    }
    pub fn messages_error(conversation_id: i32) -> Option<String> {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        if let Some(conv) = s
            .conversations
            .iter()
            .find(|c| c.conversation.id == conversation_id)
        {
            return conv.error.clone();
        }
        None
    }
    pub fn messages(conversation_id: i32) -> Vec<ClientMessage> {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        if let Some(conv) = s
            .conversations
            .iter()
            .find(|c| c.conversation.id == conversation_id)
        {
            return conv.messages.clone();
        }
        vec![]
    }
    pub fn add_message(conversation_id: i32, message: ClientMessage) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.add_message(conversation_id, message);
    }
    pub fn set_conversations(new_conversations: Vec<ConversationWithMembers>) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_conversations(new_conversations);
    }
    pub fn set_selected_conversation_id(new_id: Option<i32>) {
        {
            let mut state_lock = ConversationsState::state().write().unwrap();
            let state = state_lock.as_mut().unwrap();
            state.set_selected_conversation_id(new_id);
        }
        if new_id.is_some() {
            load_messages(new_id.unwrap());
        }
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

    pub fn set_messages(conversation_id: i32, messages: Vec<ClientMessage>) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_messages(conversation_id, messages);
    }

    pub fn set_messages_loading(conversation_id: i32, loading: bool) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_messages_loading(conversation_id, loading);
    }

    pub fn set_messages_loaded(conversation_id: i32, loaded: bool) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_messages_loaded(conversation_id, loaded);
    }

    pub fn clear_messages(conversation_id: i32) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.clear_messages(conversation_id);
    }

    pub fn clear_messages_error(conversation_id: i32) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.clear_messages_error(conversation_id);
    }

    pub fn set_messages_error(conversation_id: i32, error: String) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_messages_error(conversation_id, error);
    }

    pub fn message_draft() -> String {
        let state_lock = ConversationsState::state();
        let state = state_lock.read().unwrap();
        let s = state.as_ref().unwrap();
        return s.message_draft();
    }

    pub fn set_message_draft(draft: String) {
        let mut state_lock = ConversationsState::state().write().unwrap();
        let state = state_lock.as_mut().unwrap();
        state.set_message_draft(draft);
    }
}

fn load_conversations() {
    let loading = ConversationsState::loading();
    if loading {
        return;
    }
    ConversationsState::set_loading(true);
    thread::spawn(|| {
        let resp = fetch::<()>(
            crate::utils::fetch::ClientModes::GET,
            "/chat/conversation",
            &None,
        );
        let result = match resp {
            Ok(v) => v,
            Err(e) => {
                ConversationsState::set_error(Some(e.into()));
                ConversationsState::set_loading(false);
                return;
            }
        };
        let result_text = result.text().unwrap();
        let conversations_try_json =
            serde_json::from_str::<Response<GetConversationResponse>>(&result_text);
        match conversations_try_json {
            Ok(conversations) => {
                if !conversations.success {
                    ConversationsState::set_error(Some("Failed to fetch conversations".into()));
                } else {
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

fn load_messages(conversation_id: i32) {
    if ConversationsState::messages_loading(conversation_id)
        || ConversationsState::messages_loaded(conversation_id)
    {
        return;
    }
    ConversationsState::set_messages_loading(conversation_id, true);
    thread::spawn(move || {
        let resp = fetch::<()>(
            ClientModes::GET,
            &format!("/chat/conversation/{conversation_id}/messages"),
            &None,
        );
        match resp {
            Ok(resp) => {
                let as_text = resp.text().unwrap();
                let messages_json =
                    serde_json::from_str::<Response<GetMessagesForConversationResponse>>(&as_text);
                match messages_json {
                    Ok(data) => {
                        if data.success {
                            let messages = data.data.unwrap();
                            println!(
                                "Loaded {} messages for conversation {}",
                                messages.len(),
                                conversation_id
                            );
                            let client_messages = messages.into_iter().map(|m| m.into()).collect();
                            ConversationsState::set_messages(conversation_id, client_messages);
                            ConversationsState::set_messages_loaded(conversation_id, true);
                        } else {
                            ConversationsState::set_messages_error(
                                conversation_id,
                                if data.message.is_empty() {
                                    "Failed to load messages".into()
                                } else {
                                    data.message
                                },
                            );
                            ConversationsState::set_messages_loading(conversation_id, false);
                        }
                    }
                    Err(e) => {
                        ConversationsState::set_messages_error(conversation_id, e.to_string());
                        ConversationsState::set_messages_loading(conversation_id, false);
                    }
                }
            }
            Err(e) => {
                ConversationsState::set_messages_error(conversation_id, e.into());
                ConversationsState::set_messages_loading(conversation_id, false);
            }
        }
        UI_REBUILD_SIGNAL_SEND.get().unwrap().send(()).unwrap();
    });
}
