use crate::components::dropdown::*;
use crate::icons::filter::*;
use crate::icons::filter_off::*;
use crate::icons::search::*;
use crate::stores::topic_list_ctx::{use_topic_list_ctx, TopicListCtx};
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use web_sys::KeyboardEvent;

#[component]
pub fn TopicFilter(cx: Scope) -> impl IntoView {
    let names = TopicCategory::names();
    let names_cloned = names.clone();

    let ctx = use_context::<TopicListCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_topic_list_ctx(cx));

    let on_select = move |index: usize| {
        ctx.set_topic_category.set(index as i32);
    };

    let on_keyup = move |evt: KeyboardEvent| {
        if evt.code() == "Enter" {
            ctx.search.dispatch(());
        }
    };

    view! {
        cx,
        <div class="topic-filter">
            <div class="topic-filter__wrap">
                <input
                    class="input--sm topic-filter__input"
                    placehoder="search"
                    autocomplete="off"
                    prop:value=ctx.keyword
                    on:keyup=on_keyup
                    on:change=move |evt| (ctx.set_keyword)(event_target_value(&evt))
                />
                <button
                    class="absolute top-1/2 -translate-y-1/2 right-1"
                    on:click=move |_| ctx.search.dispatch(())
                >
                    <SearchIcon />
                </button>
            </div>
            <Dropdown data=names_cloned selected=ctx.topic_category on_select=on_select >
                <button class="btn topic-filter__trigger">
                    <FilterIcon />
                </button>
                {
                    move || ctx.topic_category.with(|&t| t != -1).then(|| view! {
                        cx,
                        <span class="topic-filter__badge" on:click=move |_| ctx.set_topic_category.set(-1) >
                            <FilterOffIcon />
                        </span>
                    })
                }
            </Dropdown>
        </div>
    }
}
