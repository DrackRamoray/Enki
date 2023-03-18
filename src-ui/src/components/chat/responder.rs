use crate::assists::scroll::scroll_to_bottom;
use crate::icons::chatgpt::*;
use crate::icons::left::*;
use crate::icons::right::*;
use enki_shared::chat::ChatMessage;
use leptos::html::Div;
use leptos::*;

pub fn render_responder_message(
    cx: Scope,
    data: Vec<ChatMessage>,
    auto_scroll: bool,
) -> HtmlElement<Div> {
    let len = data.len();
    let node_ref = create_node_ref(cx);
    let (current, set_current) = create_signal(cx, 0);

    let prev = move |_| {
        let cur = current.get();
        if cur > 0 {
            set_current.set(cur - 1);
        }
    };

    let next = move |_| {
        let cur = current.get();
        if cur < len - 1 {
            set_current.set(cur + 1);
        }
    };

    if auto_scroll {
        scroll_to_bottom(node_ref);
    }

    view! {
        cx,
        <div class="chat-list__responder">
            <div class="chat-list__avatar">
                <ChatGptIcon />
            </div>
            <ul
                class="chat-list__responder__body"
                class:multiple=move || { len > 1 }
                node_ref=node_ref
            >
                {
                    move || (len > 1).then(|| view! {
                        cx,
                        <li
                            class="chat-list__responder__switcher"
                            class:disabled=move || current.get() == 0
                            on:click=prev
                        >
                            <LeftIcon />
                        </li>
                    })
                }
                <For
                    each=move || { data.clone().into_iter().enumerate() }
                    key=|(index, _)| *index
                    view=move |cx, (index, item)| view! {
                        cx,
                        <li
                            class="chat-list__responder__message"
                            class:hidden=move || current.get() != index
                            inner_html=item.content
                        />
                    }
                />
                {
                    move || (len > 1).then(|| view! {
                        cx,
                        <li
                            class="chat-list__responder__switcher"
                            class:disabled=move || current.get() == len - 1
                            on:click=next
                        >
                            <RightIcon />
                        </li>
                    })
                }
            </ul>
        </div>
    }
}
