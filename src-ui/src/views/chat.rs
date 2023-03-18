use crate::assists::error::beautify_error;
use crate::components::chat_list_item::render_chat_list_item;
use crate::components::chat_params::*;
use crate::components::chat_record::*;
use crate::components::prompt::*;
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use crate::stores::chat_params::use_chat_params_context;
use enki_shared::chat_record::ChatRecord;
use enki_shared::topic::FetchTopic;
use enki_shared::topic_category::TopicCategory;
use leptos::html::Div;
use leptos::*;
use tauri_sys::tauri;

#[component]
pub fn Chat(cx: Scope) -> impl IntoView {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    if app_ctx.active_topic_ctg.get() != TopicCategory::Chat.to_index() {
        return view! { cx, <div></div> };
    }

    let parent_ref = create_node_ref::<Div>(cx);
    let ctx = use_chat_params_context(cx);
    provide_context(cx, ctx.clone());

    let fetcher = create_local_resource(
        cx,
        move || (),
        move |_| async move {
            let params = FetchTopic {
                id: app_ctx.active_topic_id.get(),
            };
            match tauri::invoke::<_, Vec<ChatRecord>>("plugin:chat|get_chatting_record", &params)
                .await
            {
                Ok(records) => Ok(records),
                Err(err) => {
                    error!("fetch chat record failed: {:?}", err);
                    Err(beautify_error(err))
                }
            }
        },
    );

    let send = create_action(cx, move |_: &()| {
        render_chat_list_item(cx, parent_ref);

        async move {}
    });

    view! {
        cx,
        <div class="view view--sm">
            <div class="view__body" node_ref=parent_ref >
                {
                    move || fetcher.read(cx).map(|result| match result {
                        Ok(data) => view!{ cx, <><ChatRecord data=data /></> },
                        Err(err) => view!{ cx, <>{ err }</> }
                    })
                }
            </div>
            <div class="view__footer view__footer--flex">
                <Prompt
                    sending=app_ctx.sending
                    set_sending=app_ctx.set_sending
                    message_box=ctx.message
                    set_message_box=ctx.set_message
                    send=send
                />
                <ChatParams />
            </div>
        </div>
    }
}
