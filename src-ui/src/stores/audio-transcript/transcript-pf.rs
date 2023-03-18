use crate::stores::audio_transcript_ctx::use_audio_transcript_ctx;
use crate::{assists::error::beautify_error, stores::audio_transcript_ctx::AudioTranscriptCtx};
use enki_shared::{audio::AudioParams, audio_format::AudioFormat, audio_model::AudioModel};
use leptos::*;
use tauri_sys::tauri;

#[derive(Debug, Copy, Clone)]
pub struct AudioTranscriptPfCtx {
    pub perform: Resource<(), Result<String, String>>,
}

pub fn use_audio_transcript_pf_ctx(
    cx: Scope,
    topic_id: ReadSignal<i64>,
    on_finished: Action<(), ()>,
) -> AudioTranscriptPfCtx {
    let ctx = use_context::<AudioTranscriptCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_audio_transcript_ctx(cx));

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
                language: if ctx.language.get().is_empty() {
                    None
                } else {
                    Some(ctx.language.get())
                },
                topic_id: topic_id.get(),
            };

            match tauri::invoke::<_, String>("plugin:audio_transcript|transcript_audio", &params)
                .await
            {
                Ok(data) => {
                    on_finished.dispatch(());
                    Ok(data)
                }
                Err(err) => {
                    on_finished.dispatch(());
                    Err(beautify_error(format!("{:?}", err)))
                }
            }
        },
    );

    AudioTranscriptPfCtx { perform }
}
