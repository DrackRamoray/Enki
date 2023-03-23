use crate::plugins::database::DbInstance;
use crate::{
    model::audio::Audio,
    utils::{client::create_client, error::Error},
};
use enki_shared::{audio_format::AudioFormat, audio_model::AudioModel};
use reqwest::multipart::{Form, Part};
use std::fs::read;
use tauri::State;

pub async fn trans_audio(
    db: State<'_, DbInstance>,
    file: String,
    model: AudioModel,
    prompt: String,
    fmt: AudioFormat,
    temperature: f64,
    language: Option<String>,
    topic_id: i64,
    target_url: &'static str,
) -> Result<String, Error> {
    println!("\n trans audio start ... \n ");

    let (audio_id, client) = {
        let instance = db.get_instance().lock().await;
        let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

        let audio_id = Audio::create_audio(pool, file.as_str(), prompt.as_str(), topic_id).await?;

        let client = create_client(pool).await?;

        (audio_id, client)
    };

    let audio_data = read(&file)?;

    let mut form = Form::new()
        .text("model", model.to_string())
        .text("prompt", prompt)
        .text("response_format", fmt.to_string())
        .text("temperature", temperature.to_string())
        .part("file", Part::bytes(audio_data).file_name(file));

    if let Some(lang) = language {
        form = form.text("language", lang);
    }

    let response_text = client
        .post(target_url)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    println!("\n response finished: {:?} \n", &response_text);
    println!("{}", fmt == AudioFormat::TXT);

    let text = if fmt == AudioFormat::TXT {
        format!("<code>{}</code>", response_text)
    } else {
        format!("<pre>{}</pre>", response_text)
    };

    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;
    Audio::update_audio(pool, text.as_str(), audio_id).await?;

    Ok(text)
}
