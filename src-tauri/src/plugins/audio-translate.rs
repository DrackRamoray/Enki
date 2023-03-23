use crate::plugins::database::DbInstance;
use crate::utils::audio_trans::trans_audio;
use crate::utils::constants::CHATGPT_AUDIO_TRANSLATE_URL;
use crate::{model::audio::Audio, utils::error::Error};
use enki_shared::{audio::AudioData, audio_format::AudioFormat, audio_model::AudioModel};
use tauri::{plugin::Plugin, Invoke, Runtime, State};

#[tauri::command]
async fn translate_audio(
    db: State<'_, DbInstance>,
    file: String,
    model: AudioModel,
    prompt: String,
    fmt: AudioFormat,
    temperature: f64,
    language: Option<String>, // always None
    topic_id: i64,
) -> Result<String, Error> {
    trans_audio(
        db,
        file,
        model,
        prompt,
        fmt,
        temperature,
        language,
        topic_id,
        CHATGPT_AUDIO_TRANSLATE_URL,
    )
    .await
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
