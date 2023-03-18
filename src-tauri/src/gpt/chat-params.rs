use crate::model::chat::Chat;
use crate::utils::error::Error;
use enki_shared::chat::ChatMessage;
use serde::Serialize;
use sqlx::{Pool, Sqlite};

#[derive(Serialize, Debug)]
pub struct ChatParams {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f64,
    pub top_p: f64,
    pub n: i32,
    pub max_tokens: i32,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
}

impl ChatParams {
    pub async fn gen_params(
        pool: &Pool<Sqlite>,
        model: String,
        message: String,
        instruction: Option<String>,
        temperature: f64,
        top_p: f64,
        n: i32,
        max_tokens: i32,
        presence_penalty: f64,
        frequency_penalty: f64,
        provide_previous_messages: bool,
        topic_id: i64,
    ) -> Result<Self, Error> {
        let mut messages: Vec<ChatMessage> = vec![];

        if let Some(instruction) = instruction {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: instruction,
            })
        }

        if provide_previous_messages {
            let previous_messages = Chat::retrieve_messages(pool, topic_id).await?;
            messages.extend(previous_messages);
        } else {
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: message,
            });
        }

        Ok(Self {
            model,
            messages,
            temperature,
            top_p,
            n,
            max_tokens,
            presence_penalty,
            frequency_penalty,
        })
    }
}
