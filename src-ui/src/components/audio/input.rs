use crate::assists::constants::SUPPORTED_AUDIO;
use crate::components::file_upload::*;
use crate::components::prompt::*;
use leptos::*;

#[component]
pub fn AudioInput(
    cx: Scope,
    file_path: ReadSignal<String>,
    set_file_path: WriteSignal<String>,
    sending: ReadSignal<bool>,
    set_sending: WriteSignal<bool>,
    prompt: ReadSignal<String>,
    set_prompt: WriteSignal<String>,
    send: Action<(), ()>,
) -> impl IntoView {
    view! {
        cx,
        <div class="audio-input__wrap">
            <FileUpload
                file_path=file_path
                set_file_path=set_file_path
                supported_extensions=SUPPORTED_AUDIO
            >
                "audio(mp3, mp4, mpeg, mpga, m4a, wav, or webm)"
            </FileUpload>
            <Prompt
                sending=sending
                set_sending=set_sending
                message_box=prompt
                set_message_box=set_prompt
                send=send
            />
        </div>
    }
}
