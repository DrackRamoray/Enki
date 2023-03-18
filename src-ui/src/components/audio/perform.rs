use crate::components::audio_speaker::*;
use crate::icons::chatgpt::*;
use crate::icons::trans::*;
use leptos::html::Div;
use leptos::*;

pub fn render_audio_pf(
    cx: Scope,
    parent_ref: NodeRef<Div>,
    file: String,
    prompt: String,
    perform: Resource<(), Result<String, String>>,
) {
    let el = view! {
        cx,
        <div>
            <AudioSpeaker file=file prompt=prompt auto_scroll=true />
            {
                move || match perform.read(cx) {
                    None => view! {
                        cx,
                        <ul class="audio-responder">
                            <li class="audio-responder__avatar"><ChatGptIcon /></li>
                            <li class="animate-pulse">
                                <TransIcon />
                            </li>
                        </ul>
                    },
                    Some(Ok(text)) => view! {
                        cx,
                        <ul class="audio-responder">
                            <li class="audio-responder__avatar"><ChatGptIcon /></li>
                            <li class="audio-responder__message" inner_html=text />
                        </ul>
                    },
                    Some(Err(err)) => view! {
                        cx,
                        <ul class="audio-responder">
                            <li class="audio-responder__avatar"><ChatGptIcon /></li>
                            <li>
                                { err }
                            </li>
                        </ul>
                    },
                }
            }
        </div>
    };

    let parent_el = parent_ref.get().unwrap();

    let _ = parent_el.append_child(&el);
}
