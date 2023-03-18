use crate::assists::error::beautify_error;
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use crate::stores::chat_params::{use_chat_params_context, ChatParamsContext};
use enki_shared::chat::{ChatMessage, NewChat};
use enki_shared::chat_model::ChatModel;
use leptos::*;
use tauri_sys::tauri;

#[derive(Copy, Clone)]
pub struct ChatCreatorContext {
    pub creator: Resource<(), Result<Vec<ChatMessage>, String>>,
}

pub fn use_chat_creator_context(cx: Scope) -> ChatCreatorContext {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));
    let ctx = use_context::<ChatParamsContext>(cx);
    let ctx = ctx.unwrap_or_else(|| use_chat_params_context(cx));

    let creator = create_resource(
        cx,
        move || (),
        move |_| async move {
            let topic_id = app_ctx.active_topic_id.get();

            let instruction = ctx.instruction.get();
            let instruction = if instruction.len() == 0 {
                None
            } else {
                Some(instruction)
            };

            let params = NewChat {
                message: ctx.message.get(),
                model: ChatModel::value_from_index(ctx.model.get()),
                instruction,
                temperature: ctx.temperature.get(),
                top_p: ctx.top_p.get(),
                n: ctx.n.get(),
                max_tokens: ctx.max_tokens.get(),
                presence_penalty: ctx.presence_penalty.get(),
                frequency_penalty: ctx.frequency_penalty.get(),
                provide_previous_messages: ctx.provide_previous_messages.get(),
                topic_id,
            };

            match tauri::invoke::<_, Vec<ChatMessage>>("plugin:chat|start_chatting", &params).await
            {
                Ok(responses) => {
                    app_ctx.set_sending.set(false);
                    Ok(responses)
                }
                Err(err) => {
                    app_ctx.set_sending.set(false);
                    error!("x starting chatting failed: {:?}", err);
                    Err(beautify_error(err))
                }
            }
        },
    );

    ChatCreatorContext { creator }
}
