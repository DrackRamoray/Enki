use crate::chat::ChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatRecord {
    pub speaker: ChatMessage,
    pub responder: Vec<ChatMessage>,
}
