use enki_shared::{audio::AudioData, audio_format::AudioFormat, audio_model::AudioModel};
use reqwest::multipart::{Form, Part};
use std::fs::read;
use tauri::{plugin::Plugin, Invoke, Runtime, State};

use crate::{
    model::audio::Audio,
    utils::{client::create_client, constants::CHATGPT_AUDIO_TRANSLATE_URL, error::Error},
};

use super::database::DbInstance;

#[tauri::command]
async fn translate_audio(
    db: State<'_, DbInstance>,
    file: String,
    model: AudioModel,
    prompt: String,
    fmt: AudioFormat,
    temperature: f64,
    _language: Option<String>, // alaways None
    topic_id: i64,
) -> Result<String, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    println!("\n translate audio start ... \n ");

    let audio_id = Audio::create_audio(pool, file.as_str(), prompt.as_str(), topic_id).await?;

    let client = create_client(pool).await?;

    let audio_data = read(&file)?;

    let form = Form::new()
        .text("model", model.to_string())
        .text("prompt", prompt)
        .text("response_format", fmt.to_string())
        .text("temperature", temperature.to_string())
        .part("file", Part::bytes(audio_data).file_name(file));

    let response_text = client
        .post(CHATGPT_AUDIO_TRANSLATE_URL)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    println!("\n response finished: {:?} \n", &response_text);

    let text = if fmt == AudioFormat::TXT {
        format!("<code>{}</code>", response_text)
    } else {
        format!("<pre>{}</pre>", response_text)
    };

    Audio::update_audio(pool, text.as_str(), audio_id).await?;

    Ok(text)
}

#[tauri::command]
async fn get_audio_translate_record(
    db: State<'_, DbInstance>,
    id: i64,
) -> Result<Vec<AudioData>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    Audio::retrieve_records(pool, id).await
}

pub struct AudioTranslatePlugin<R: Runtime> {
    handler: Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>,
}

impl<R: Runtime> AudioTranslatePlugin<R> {
    pub fn new() -> Self {
        Self {
            handler: Box::new(tauri::generate_handler![
                translate_audio,
                get_audio_translate_record
            ]),
        }
    }
}

impl<R: Runtime> Plugin<R> for AudioTranslatePlugin<R> {
    fn name(&self) -> &'static str {
        "audio_translate"
    }

    fn extend_api(&mut self, invoke: Invoke<R>) -> bool {
        (self.handler)(invoke)
    }
}
