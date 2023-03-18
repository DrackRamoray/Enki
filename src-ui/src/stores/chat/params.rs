use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use leptos::*;

#[derive(Copy, Clone)]
pub struct ChatParamsContext {
    pub message: ReadSignal<String>,
    pub set_message: WriteSignal<String>,
    pub instruction: ReadSignal<String>,
    pub set_instruction: WriteSignal<String>,
    pub model: ReadSignal<usize>,
    pub set_model: WriteSignal<usize>,
    pub temperature: ReadSignal<f64>,
    pub set_temperature: WriteSignal<f64>,
    pub top_p: ReadSignal<f64>,
    pub set_top_p: WriteSignal<f64>,
    pub n: ReadSignal<i32>,
    pub set_n: WriteSignal<i32>,
    pub max_tokens: ReadSignal<i32>,
    pub set_max_tokens: WriteSignal<i32>,
    pub presence_penalty: ReadSignal<f64>,
    pub set_presence_penalty: WriteSignal<f64>,
    pub frequency_penalty: ReadSignal<f64>,
    pub set_frequency_penalty: WriteSignal<f64>,
    pub provide_previous_messages: ReadSignal<bool>,
    pub set_provide_previous_messages: WriteSignal<bool>,
}

pub fn use_chat_params_context(cx: Scope) -> ChatParamsContext {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    let (message, set_message) = create_signal(cx, String::new());
    let (instruction, set_instruction) = create_signal(cx, String::new());
    let (model, set_model) = create_signal(cx, 0);
    let (temperature, set_temperature) = create_signal(cx, 1.0);
    let (top_p, set_top_p) = create_signal(cx, 1.0);
    let (n, set_n) = create_signal(cx, 1);
    let (max_tokens, set_max_tokens) = create_signal(cx, 1024);
    let (presence_penalty, set_presence_penalty) = create_signal(cx, 0.0);
    let (frequency_penalty, set_frequency_penalty) = create_signal(cx, 0.0);
    let (provide_previous_messages, set_provide_previous_messages) = create_signal(cx, true);

    create_effect(cx, move |_| {
        let _topic_id = app_ctx.active_topic_id.get();

        set_instruction.set(String::new());
        set_model.set(0);
        set_temperature.set(1.0);
        set_top_p.set(1.0);
        set_n.set(1);
        set_max_tokens.set(1024);
        set_presence_penalty(0.0);
        set_frequency_penalty(0.0);
        set_provide_previous_messages(true);
    });

    ChatParamsContext {
        message,
        set_message,
        instruction,
        set_instruction,
        model,
        set_model,
        temperature,
        set_temperature,
        top_p,
        set_top_p,
        n,
        set_n,
        max_tokens,
        set_max_tokens,
        presence_penalty,
        set_presence_penalty,
        frequency_penalty,
        set_frequency_penalty,
        provide_previous_messages,
        set_provide_previous_messages,
    }
}
