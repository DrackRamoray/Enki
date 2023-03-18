use crate::components::audio_speaker::*;
use crate::icons::chatgpt::*;
use crate::icons::trans_error::*;
use enki_shared::audio::AudioData;
use leptos::*;

#[component]
pub fn AudioRecord(cx: Scope, data: Vec<AudioData>) -> impl IntoView {
    let render_response = move |response: String| match response.is_empty() {
        true => view! { cx, <li><TransErrorIcon /></li> },
        false => view! { cx, <li class="audio-responder__message" inner_html=response /> },
    };

    view! {
        cx,
        <For
            each=move || data.clone().into_iter().enumerate()
            key=|(index, _)| *index
            view=move |cx, (_, item)| {
                view! {
                    cx,
                    <div>
                        <AudioSpeaker file=item.file prompt=item.prompt auto_scroll=false />
                        <ul class="audio-responder">
                            <li class="audio-responder__avatar"><ChatGptIcon /></li>
                            {
                                let response = item.response.clone();

                                render_response(response)
                            }
                        </ul>
                    </div>
                }
            }
        />
    }
}
