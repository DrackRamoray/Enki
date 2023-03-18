use crate::assists::error::beautify_error;
use crate::components::loading::*;
use crate::components::topic_filter::*;
use crate::components::topic_list::{
    render_topic_list, render_topic_list_empty, render_topic_list_error,
};
use crate::stores::topic_list_ctx::use_topic_list_ctx;
use leptos::*;

#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    let ctx = use_topic_list_ctx(cx);
    provide_context(cx, ctx);

    view! {
        cx,
        <div class="sticky top-0">
            <TopicFilter />
        </div>
        {
            move || match ctx.fetcher.read(cx) {
                None => view! { cx, <><Loading /></> },
                Some(Ok(topic_list)) => match topic_list.is_empty() {
                    true => view! {cx, <> { render_topic_list_empty(cx) }</> },
                    false => view! { cx, <>{ render_topic_list(cx, topic_list) }</> },
                },
                Some(Err(err)) => view! { cx, <> { render_topic_list_error(cx, beautify_error(err)) } </> },
            }
        }
    }
}
