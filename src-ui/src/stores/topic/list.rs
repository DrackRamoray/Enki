use crate::assists::error::beautify_error;
use enki_shared::topic::{SearchTopicList, Topic};
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use tauri_sys::tauri;

#[derive(Clone, Copy)]
pub struct TopicListCtx {
    pub topic_category: ReadSignal<i32>,
    pub set_topic_category: WriteSignal<i32>,
    pub keyword: ReadSignal<String>,
    pub set_keyword: WriteSignal<String>,
    pub search: Action<(), ()>,
    pub fetcher: Resource<(i32, i32), Result<Vec<Topic>, String>>,
}

pub fn use_topic_list_ctx(cx: Scope) -> TopicListCtx {
    let (topic_category, set_topic_category) = create_signal(cx, -1);
    let (keyword, set_keyword) = create_signal(cx, String::new());
    let (throttle, set_throttle) = create_signal(cx, -1);

    let fetcher = create_resource(
        cx,
        move || (topic_category.get(), throttle.get()),
        move |_| async move {
            let title = keyword.get();
            let title = if title.len() == 0 { None } else { Some(title) };

            let category = topic_category.get();
            let category = if category == -1 {
                None
            } else {
                Some(TopicCategory::value_from_index(category as usize))
            };

            match tauri::invoke::<_, Vec<Topic>>(
                "plugin:topic|get_topic_list",
                &SearchTopicList { title, category },
            )
            .await
            {
                Ok(topic_list) => Ok(topic_list),
                Err(err) => {
                    log!("get topic list failed: {:?}", err);
                    Err(beautify_error(err))
                }
            }
        },
    );

    let search = create_action(cx, move |_| async move {
        set_throttle.update(|v| *v *= -1);
    });

    window_event_listener("fresh_topic_list", move |_| {
        fetcher.refetch();
    });

    TopicListCtx {
        topic_category,
        set_topic_category,
        keyword,
        set_keyword,
        search,
        fetcher,
    }
}
