use crate::assists::error::beautify_error;
use crate::assists::notify::{notify, NotifyType};
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use leptos_router::*;

pub fn location_to_topic(cx: Scope, tc: TopicCategory, id: i64) {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    let navigate = use_navigate(cx);
    let path = tc.to_path();
    let index = tc.to_index();

    if let Err(err, ..) = navigate(path.as_str(), Default::default()) {
        notify(cx, NotifyType::Error, beautify_error(err));
    }

    app_ctx.set_active_topic_ctg.set(index);
    app_ctx.set_active_topic_id.set(id);
    app_ctx.set_sending.set(false);
}

pub fn location_to_home(cx: Scope) {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    let navigate = use_navigate(cx);

    if let Err(err) = navigate("/", Default::default()) {
        notify(cx, NotifyType::Error, beautify_error(err));
        return;
    }

    app_ctx.set_active_topic_id.set(0);
    app_ctx.set_active_topic_ctg.set(-1);
    app_ctx.set_sending.set(false);
}

pub fn location_to_topic_create(cx: Scope) {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    let navigate = use_navigate(cx);

    if let Err(err) = navigate("/topic/create", Default::default()) {
        notify(cx, NotifyType::Error, beautify_error(err));
        return;
    }

    app_ctx.set_active_topic_id.set(0);
    app_ctx.set_active_topic_ctg.set(-1);
    app_ctx.set_sending.set(false);
}
