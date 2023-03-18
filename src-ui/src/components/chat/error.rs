use crate::icons::chatgpt::*;
use leptos::html::Div;
use leptos::*;

pub fn render_responder_message_with_error(cx: Scope, err: String) -> HtmlElement<Div> {
    view! {
        cx,
        <div class="chat-list__responder">
            <div class="chat-list__avatar">
                <ChatGptIcon />
            </div>
            <div inner_html=err />
        </div>
    }
}
