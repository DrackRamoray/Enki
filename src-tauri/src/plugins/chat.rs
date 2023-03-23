use crate::gpt::chat_params::ChatParams;
use crate::model::chat::Chat;
use crate::model::chat_response::ChatResponse;
use crate::plugins::database::DbInstance;
use crate::utils::client::create_client;
use crate::utils::constants::CHATGPT_CHAT_COMPLETIONS_URL;
use crate::utils::error::Error;
use crate::utils::md_to_html::convert_to_html;
use enki_shared::chat::{ChatMessage, ChatResponse as ChatResponseData};
use enki_shared::chat_model::ChatModel;
use enki_shared::chat_record::ChatRecord;
use tauri::plugin::Plugin;
use tauri::{Invoke, Runtime, State};

#[tauri::command]
async fn start_chatting(
    db: State<'_, DbInstance>,
    message: String,
    model: ChatModel,
    instruction: Option<String>,
    temperature: f64,
    top_p: f64,
    n: i32,
    max_tokens: i32,
    presence_penalty: f64,
    frequency_penalty: f64,
    provide_previous_messages: bool,
    topic_id: i64,
) -> Result<Vec<ChatMessage>, Error> {
    let (chat_id, client, req_params) = {
        let instance = db.get_instance().lock().await;
        let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

        let chat_id = Chat::create_chat(pool, "user", message.as_str(), topic_id).await?;

        let req_params = ChatParams::gen_params(
            pool,
            model.to_string(),
            message,
            instruction,
            temperature,
            top_p,
            n,
            max_tokens,
            presence_penalty,
            frequency_penalty,
            provide_previous_messages,
            topic_id,
        )
        .await?;

        let client = create_client(pool).await?;

        (chat_id, client, req_params)
    };

    print!("\n params = {:?} \n", req_params);

    let response_text = client
        .post(CHATGPT_CHAT_COMPLETIONS_URL)
        .json(&req_params)
        .send()
        .await?
        .text()
        .await?;

    println!("\n response text = {:?} \n", response_text);

    let choices = match serde_json::from_str::<ChatResponseData>(response_text.as_str())? {
        ChatResponseData::Ok { choices } => choices,
        ChatResponseData::Err { error } => return Err(Error::ChatFailed(error.message)),
    };

    let mut ans = Vec::new();

    for c in choices {
        let markdown = convert_to_html(c.message.content.as_str());

        {
            let instance = db.get_instance().lock().await;
            let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

            ChatResponse::create_response(
                pool,
                c.message.role.as_str(),
                c.message.content.as_str(),
                markdown.as_str(),
                chat_id,
                topic_id,
            )
            .await?;
        }

        ans.push(ChatMessage {
            role: c.message.role,
            content: markdown,
        });
    }

    Ok(ans)
}

#[tauri::command]
async fn get_chatting_record(db: State<'_, DbInstance>, id: i64) -> Result<Vec<ChatRecord>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    Chat::retrieve_records(pool, id).await
}

pub struct ChatPlugin<R: Runtime>(Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>);

impl<R: Runtime> ChatPlugin<R> {
    pub fn new() -> Self {
        Self(Box::new(tauri::generate_handler![
            start_chatting,
            get_chatting_record,
        ]))
    }
}

impl<R: Runtime> Plugin<R> for ChatPlugin<R> {
    fn name(&self) -> &'static str {
        "chat"
    }

    fn extend_api(&mut self, invoke: Invoke<R>) -> bool {
        (self.0)(invoke)
    }
}
