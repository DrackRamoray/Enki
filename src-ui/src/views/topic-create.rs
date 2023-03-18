use crate::components::dropdown::*;
use crate::icons::enter::*;
use crate::icons::selector::*;
use crate::stores::topic_create_ctx::use_topic_create_ctx;
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use leptos_router::*;

#[component]
pub fn TopicCreate(cx: Scope) -> impl IntoView {
    let names = TopicCategory::names();
    let names_cloned = names.clone();

    let (title, set_title) = create_signal(cx, String::new());
    let (selected, set_selected) = create_signal(cx, -1);
    let display_value = Signal::derive(cx, move || {
        let idx = selected.get();
        if idx == -1 {
            String::new()
        } else {
            names[idx as usize].clone()
        }
    });

    let query_map = use_query_map(cx);
    let get_selected_index = move || {
        query_map.with(|q| match q.get("topic") {
            Some(topic) => set_selected(topic.parse::<i32>().unwrap()),
            None => {}
        });
    };

    let on_select = move |index: usize| {
        set_selected(index as i32);
    };

    let ctx = use_topic_create_ctx(cx, selected, title);

    request_animation_frame(move || get_selected_index());

    view! {
        cx,
        <div class="flex items-center flex-col pt-20">
            <div class="w-64 relative">
                <Dropdown data=names_cloned selected=selected on_select=on_select >
                    <div class="topic-create__select">
                        <input type="checkbox" class="topic-create__select__input peer" />
                        <div class="topic-create__select__display">
                            <div class="uppercase">{ move || display_value.get() }</div>
                            <div class="topic-create__select__indicator">
                                <SelectorIcon />
                            </div>
                        </div>
                    </div>
                </Dropdown>
            </div>
            <div class="w-64 mt-8">
                <input
                    class="input w-full"
                    placeholder="topic name"
                    prop:value=title
                    on:change=move |evt| set_title(event_target_value(&evt))
                />
            </div>
            <div class="w-64 mt-8 text-right">
                <button class="btn" on:click=move |_| ctx.submit.dispatch(()) >
                    <EnterIcon />
                </button>
            </div>
        </div>
    }
}
