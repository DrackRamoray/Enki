use leptos::*;

#[derive(Clone, Copy)]
pub struct AppCtx {
    pub sending: ReadSignal<bool>,
    pub set_sending: WriteSignal<bool>,
    pub active_topic_id: ReadSignal<i64>,
    pub set_active_topic_id: WriteSignal<i64>,
    pub active_topic_ctg: ReadSignal<i32>,
    pub set_active_topic_ctg: WriteSignal<i32>,
}

pub fn use_app_ctx(cx: Scope) -> AppCtx {
    let (sending, set_sending) = create_signal(cx, false);
    let (active_topic_id, set_active_topic_id) = create_signal(cx, 0);
    let (active_topic_ctg, set_active_topic_ctg) = create_signal(cx, -1);

    AppCtx {
        sending,
        set_sending,
        active_topic_id,
        set_active_topic_id,
        active_topic_ctg,
        set_active_topic_ctg,
    }
}
