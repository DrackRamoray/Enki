use crate::components::topic_item::*;
use crate::icons::empty::*;
use crate::icons::error::*;
use crate::icons::refresh::*;
use crate::stores::topic_list_ctx::TopicListCtx;
use enki_shared::topic::Topic;
use leptos::html::Ul;
use leptos::*;

pub fn render_topic_list(cx: Scope, data: Vec<Topic>) -> HtmlElement<Ul> {
    view! {
        cx,
        <ul>
            <For
                 each=move || { data.clone().into_iter() }
                 key=|item| item.id
                 view=move |cx, item| view! {
                     cx,
                     <TopicItem item=item />
                 }
             />
        </ul>
    }
}

pub fn render_topic_list_empty(cx: Scope) -> HtmlElement<Ul> {
    view! {
        cx,
        <ul class="topic-list">
            <li class="text-center"><EmptyIcon /></li>
            <li class="text-center">"No Data"</li>
        </ul>
    }
}

pub fn render_topic_list_error(cx: Scope, error_message: impl IntoView) -> HtmlElement<Ul> {
    let ctx = use_context::<TopicListCtx>(cx);

    view! {
        cx,
        <ul class="topic-list">
            <li class="text-center"><ErrorIcon /></li>
            <li class="text-center">"Error"</li>
            <li>{ error_message }</li>
            {
                move || match ctx {
                    Some(ctx) => view! {
                        cx,
                        <li>
                            <button class="btn" on:click=move |_| ctx.fetcher.refetch()>
                                <RefreshIcon />
                            </button>
                        </li>
                    },
                    None => view! { cx, <li></li>},
                }
            }
        </ul>
    }
}
