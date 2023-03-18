use crate::chat_model::ChatModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChat {
    pub message: String,
    pub model: ChatModel,
    pub instruction: Option<String>,
    pub temperature: f64,
    pub top_p: f64,
    pub n: i32,
    pub max_tokens: i32,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
    pub provide_previous_messages: bool,
    pub topic_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatResponse {
    Ok { choices: Vec<ChatMessageObj> },
    Err { error: ChatResponseError },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessageObj {
    pub message: ChatMessage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatResponseError {
    pub message: String,
}
