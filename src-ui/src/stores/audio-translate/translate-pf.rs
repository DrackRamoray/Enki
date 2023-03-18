use enki_shared::{audio::AudioParams, audio_format::AudioFormat, audio_model::AudioModel};
use leptos::*;
use tauri_sys::tauri;

use crate::assists::error::beautify_error;

use crate::stores::audio_translate_ctx::{use_audio_translate_ctx, AudioTranslateCtx};

#[derive(Copy, Clone)]
pub struct AudioTranslatePfCtx {
    pub perform: Resource<(), Result<String, String>>,
}

pub fn use_audio_translate_pf_ctx(
    cx: Scope,
    topic_id: ReadSignal<i64>,
    on_finished: Action<(), ()>,
) -> AudioTranslatePfCtx {
    let ctx = use_context::<AudioTranslateCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_audio_translate_ctx(cx));

    let perform = create_resource(
        cx,
        move || (),
        move |_| async move {
            let params = AudioParams {
                file: ctx.file.get(),
                model: AudioModel::value_from_index(ctx.model.get()),
                prompt: ctx.prompt.get(),
                fmt: AudioFormat::value_from_index(ctx.fmt.get()),
                temperature: ctx.temperature.get(),
                language: None,
                topic_id: topic_id.get(),
            };

            match tauri::invoke::<_, String>("plugin:audio_translate|translate_audio", &params)
                .await
            {
                Ok(data) => {
                    on_finished.dispatch(());
                    Ok(data)
                }
                Err(err) => {
                    on_finished.dispatch(());
                    Err(beautify_error(err))
                }
            }
        },
    );

    AudioTranslatePfCtx { perform }
}
