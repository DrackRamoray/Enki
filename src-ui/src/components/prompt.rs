use crate::icons::prompt_send::*;
use leptos::html::Div;
use leptos::*;
use std::time::Duration;
use web_sys::KeyboardEvent;

#[component]
pub fn Prompt(
    cx: Scope,
    sending: ReadSignal<bool>,
    set_sending: WriteSignal<bool>,
    message_box: ReadSignal<String>,
    set_message_box: WriteSignal<String>,
    send: Action<(), ()>,
) -> impl IntoView {
    let message_ref = create_node_ref(cx);

    let disabled = Signal::derive(cx, move || sending.get() || message_box.get().is_empty());

    let on_message_input = move |_| {
        let editor: HtmlElement<Div> = message_ref.get().unwrap();
        let text = editor.inner_text().trim().to_string();
        set_message_box.set(text);
    };

    let on_send = move || {
        set_sending.set(true);
        send.dispatch(());
        set_timeout(
            move || {
                let editor: HtmlElement<Div> = message_ref.get().unwrap();
                editor.set_text_content(None);
                set_message_box.set(String::new());
            },
            Duration::from_millis(50),
        );
    };

    let on_keyup = move |evt: KeyboardEvent| {
        if !sending.get() && evt.code() == "Enter" && !evt.shift_key() {
            on_send()
        }
    };

    view! {
        cx,
        <div class="min-w-full max-w-full relative">
            <div
                class="prompt__editor"
                contenteditable
                node_ref=message_ref
                on:keyup=on_keyup
                on:input=on_message_input
            >
            </div>
            <button
                class="prompt__send-btn"
                prop:disabled=disabled
                on:click=move |_| on_send()
            >
                <PromptSendIcon />
            </button>
        </div>
    }
}
