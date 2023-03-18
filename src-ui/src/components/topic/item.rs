use crate::icons::confirm::*;
use crate::icons::edit::*;
use crate::icons::edit_off::*;
use crate::icons::trash::*;
use crate::stores::topic_item_ctx::use_topic_item_ctx;
use enki_shared::topic::Topic;
use leptos::html::Li;
use leptos::*;
use web_sys::KeyboardEvent;

#[component]
pub fn TopicItem(cx: Scope, item: Topic) -> impl IntoView {
    let current_id = item.id;
    let ctx = use_topic_item_ctx(cx, item);

    view! {
        cx,
        <>
            {
                move || match ctx.editing.get() {
                    true => render_editing(cx, ctx.title, ctx.set_title, ctx.confirm, ctx.cancel),
                    false => render_viewing(cx, ctx.active_id, current_id, ctx.title, ctx.set_editing, ctx.jump, ctx.remove),
                }
            }
        </>
    }
}

fn render_editing(
    cx: Scope,
    title: ReadSignal<String>,
    set_title: WriteSignal<String>,
    confirm: Action<(), ()>,
    cancel: Action<(), ()>,
) -> HtmlElement<Li> {
    let on_keyup = move |evt: KeyboardEvent| {
        if evt.code() == "Enter" {
            confirm.dispatch(());
        }
    };

    view! {
        cx,
        <li class="topic-item">
            <input
                class="topic-item__input input"
                prop:value=title
                on:keyup=on_keyup
                on:input=move |evt| set_title(event_target_value(&evt))
            />
            <div class="topic-item__btn-group hidden">
                <button on:click=move |_| confirm.dispatch(()) >
                    <ConfirmIcon />
                </button>
                <button on:click=move |_| cancel.dispatch(()) >
                    <EditOffIcon />
                </button>
            </div>
        </li>
    }
}

fn render_viewing(
    cx: Scope,
    active_id: Signal<i64>,
    current_id: i64,
    title: ReadSignal<String>,
    set_editing: WriteSignal<bool>,
    jump: Action<(), ()>,
    remove: Action<(), ()>,
) -> HtmlElement<Li> {
    view! {
        cx,
        <li class="topic-item" class:active=move || active_id.get() == current_id >
            <a
                class="topic-item__title"
                href="#"
                on:click=move |_| jump.dispatch(())
            >
                { title.get() }
            </a>
            <div class="topic-item__btn-group hidden">
                <button on:click=move |_| set_editing(true) >
                    <EditIcon />
                </button>
                <button on:click=move |_| remove.dispatch(())>
                    <TrashIcon />
                </button>
            </div>
        </li>
    }
}
