use crate::assists::constants::SUPPORTED_LAN;
use crate::assists::error::beautify_error;
use crate::components::audio_input::*;
use crate::components::audio_params::*;
use crate::components::audio_perform::render_audio_pf;
use crate::components::audio_record::*;
use crate::components::auto_complete::*;
use crate::components::error_warning::*;
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use crate::stores::audio_transcript_ctx::use_audio_transcript_ctx;
use crate::stores::audio_transcript_pf_ctx::use_audio_transcript_pf_ctx;
use enki_shared::audio::{AudioData, AudioRecordParams};
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use std::time::Duration;
use tauri_sys::tauri;

#[component]
pub fn AudioTranscript(cx: Scope) -> impl IntoView {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    if app_ctx.active_topic_ctg.get() != TopicCategory::AudioTranscript.to_index() {
        return view! { cx, <div></div> };
    }

    let parent_ref = create_node_ref(cx);
    let ctx = use_audio_transcript_ctx(cx);

    provide_context(cx, ctx);

    let on_finished = create_action(cx, move |_: &()| {
        app_ctx.set_sending.set(false);

        async move {}
    });

    let send = create_action(cx, move |_: &()| async move {
        let pf_ctx = use_audio_transcript_pf_ctx(cx, app_ctx.active_topic_id, on_finished);

        render_audio_pf(
            cx,
            parent_ref,
            ctx.file.get(),
            ctx.prompt.get(),
            pf_ctx.perform,
        );

        set_timeout(
            move || {
                (ctx.set_file)(String::new());
                (ctx.set_prompt)(String::new());
            },
            Duration::from_millis(20),
        );
    });

    let fetcher = create_resource(
        cx,
        move || (),
        move |_| async move {
            let params = AudioRecordParams {
                id: app_ctx.active_topic_id.get(),
            };

            match tauri::invoke::<_, Vec<AudioData>>(
                "plugin:audio_transcript|get_audio_transcript_record",
                &params,
            )
            .await
            {
                Ok(data) => Ok(data),
                Err(err) => Err(beautify_error(err)),
            }
        },
    );

    view! {
        cx,
        <div class="view view--md">
            <div class="view__body" node_ref=parent_ref>
                {
                    move || fetcher.read(cx).map(|result| match result {
                        Ok(data) => view!{ cx, <><AudioRecord data=data /></> },
                        Err(err) => view!{ cx, <><ErrorWarning err=err /></> }
                    })
                }
            </div>
            <div class="view__footer view__footer--flex audio__footer">
                <AudioInput
                    file_path=ctx.file
                    set_file_path=ctx.set_file
                    sending=app_ctx.sending
                    set_sending=app_ctx.set_sending
                    prompt=ctx.prompt
                    set_prompt=ctx.set_prompt
                    send=send
                />
                <AudioParams
                    temperature=ctx.temperature
                    set_temperature=ctx.set_temperature
                    fmt=ctx.fmt
                    set_fmt=ctx.set_fmt
                >
                    <AutoComplete
                        text=ctx.language
                        set_text=ctx.set_language
                        data=SUPPORTED_LAN
                    />
                </AudioParams>
            </div>
        </div>
    }
}
