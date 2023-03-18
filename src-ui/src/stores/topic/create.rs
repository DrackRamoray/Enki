use crate::assists::error::beautify_error;
use crate::assists::location::location_to_topic;
use crate::assists::notify::notify;
use crate::assists::notify::NotifyType;
use enki_shared::topic::{NewTopic, Topic};
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use tauri_sys::tauri;
use web_sys::Event;

pub struct TopicCreateCtx {
    pub submit: Action<(), ()>,
}

pub fn use_topic_create_ctx(
    cx: Scope,
    selected: ReadSignal<i32>,
    title: ReadSignal<String>,
) -> TopicCreateCtx {
    let submit = create_action(cx, move |_| {
        let cx = cx.clone();

        async move {
            let i = selected.get();
            let t = title.get().trim().to_string();

            if i == -1 {
                notify(cx, NotifyType::Warn, "topic category cant not be empty");
                return;
            }

            if t.is_empty() {
                notify(cx, NotifyType::Warn, "topic name can not be empty");
                return;
            }

            match tauri::invoke::<_, Topic>(
                "plugin:topic|create_topic",
                &NewTopic {
                    title: t,
                    category: TopicCategory::value_from_index(i as usize),
                },
            )
            .await
            {
                Ok(topic) => {
                    if let Ok(evt) = Event::new("fresh_topic_list") {
                        if let Ok(b) = window().dispatch_event(&evt) {
                            if b {
                                location_to_topic(cx, topic.category, topic.id);
                                return;
                            }
                        }
                    }

                    notify(cx, NotifyType::Warn, "Fresh topic list failed.");
                }
                Err(err) => {
                    error!("create topic failed: {:?}", err);
                    notify(cx, NotifyType::Warn, beautify_error(err));
                }
            }
        }
    });

    TopicCreateCtx { submit }
}
