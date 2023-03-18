use crate::components::chat_list_responder::render_responder_message;
use crate::components::prompt_speaker::render_prompt_speaker;
use enki_shared::chat_record::ChatRecord;
use leptos::*;

#[component]
pub fn ChatRecord(cx: Scope, data: Vec<ChatRecord>) -> impl IntoView {
    view! {
        cx,
        <For
            each=move || data.clone().into_iter().enumerate()
            key=|(index, _)| *index
            view=move |cx, (_, item)| {
                let content = item.speaker.content;
                let responses = item.responder;
                let node_ref = create_node_ref(cx);

                view! {
                    cx,
                    <div>
                        {
                            move || render_prompt_speaker(cx, node_ref, content.clone())
                        }
                        {
                            move || render_responder_message(cx, responses.clone(), false)
                        }
                    </div>
                }
            }
        />
    }
}
