use crate::assists::error::beautify_error;
use crate::assists::location::location_to_topic;
use crate::assists::notify::{notify, NotifyType};
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use crate::stores::topic_list_ctx::{use_topic_list_ctx, TopicListCtx};
use enki_shared::topic::{EditTopic, RemoveTopic, Topic};
use leptos::*;
use leptos_router::*;
use tauri_sys::tauri;

pub struct TopicItemCtx {
    pub active_id: Signal<i64>,
    pub editing: ReadSignal<bool>,
    pub set_editing: WriteSignal<bool>,
    pub title: ReadSignal<String>,
    pub set_title: WriteSignal<String>,
    pub jump: Action<(), ()>,
    pub confirm: Action<(), ()>,
    pub cancel: Action<(), ()>,
    pub remove: Action<(), ()>,
}

pub fn use_topic_item_ctx(cx: Scope, item: Topic) -> TopicItemCtx {
    let current_id = item.id;
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));
    let ctx = use_context::<TopicListCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_topic_list_ctx(cx));

    let active_id = Signal::derive(cx, move || app_ctx.active_topic_id.get());
    let (editing, set_editing) = create_signal(cx, false);
    let (title, set_title) = create_signal(cx, item.title.clone());
    let (temp, set_temp) = create_signal(cx, item.title);

    let jump = create_action(cx, move |_| {
        let category = item.category;
        let id = item.id;
        location_to_topic(cx, category, id);

        async move {}
    });

    let confirm = create_action(cx, move |_| async move {
        match tauri::invoke::<_, ()>(
            "plugin:topic|edit_topic",
            &EditTopic {
                id: current_id,
                title: title.get(),
            },
        )
        .await
        {
            Ok(()) => {
                set_editing(false);
                set_temp(title.get());
                ctx.fetcher.refetch();
            }
            Err(err) => {
                log!("edit topic failed: {:?}", err);
                notify(cx, NotifyType::Error, beautify_error(err));
            }
        }
    });

    let cancel = create_action(cx, move |_| async move {
        set_title(temp.get());
        set_editing(false);
    });

    let remove = create_action(cx, move |_| async move {
        match tauri::invoke::<_, ()>("plugin:topic|remove_topic", &RemoveTopic { id: current_id })
            .await
        {
            Ok(()) => {
                let navigate = use_navigate(cx);

                ctx.fetcher.refetch();

                if current_id == active_id.get() {
                    if let Err(err) = navigate("/", Default::default()) {
                        notify(cx, NotifyType::Warn, beautify_error(err));
                    }
                }
            }
            Err(err) => {
                log!("remove topic failed: {:?}", err);
                notify(cx, NotifyType::Error, beautify_error(err));
            }
        }
    });

    TopicItemCtx {
        active_id,
        editing,
        set_editing,
        title,
        set_title,
        jump,
        confirm,
        cancel,
        remove,
    }
}
