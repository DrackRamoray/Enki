use crate::components::chat_list_error::render_responder_message_with_error;
use crate::components::chat_list_responder::render_responder_message;
use crate::components::chat_list_speaker::*;
use crate::icons::chatgpt::*;
use crate::stores::chat_create::use_chat_creator_context;
use leptos::html::Div;
use leptos::*;

pub fn render_chat_list_item(cx: Scope, parent_ref: NodeRef<Div>) {
    let ctx = use_chat_creator_context(cx);

    let el = view! {
        cx,
        <div class="pr-2">
            <SpeakerMessage />
            {
                move || match ctx.creator.read(cx) {
                    None => render_loading(cx),
                    Some(Ok(data)) => render_responder_message(cx, data, true),
                    Some(Err(err)) => render_responder_message_with_error(cx, err),
                }
            }
        </div>
    };

    let parent = parent_ref.get().unwrap();
    let _ = parent.append_with_node_1(&el);
}

fn render_loading(cx: Scope) -> HtmlElement<Div> {
    view! {
        cx,
        <div class="chat-list__responder">
            <div class="chat-list__avatar">
                <ChatGptIcon />
            </div>
            <div class="chat-list--loading"></div>
        </div>
    }
}
