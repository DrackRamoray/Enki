use crate::assists::scroll::scroll_to_bottom;
use crate::icons::chatgpt::*;
use crate::icons::image_broken::*;
use crate::icons::image_editing::*;
use crate::icons::left::*;
use crate::icons::right::*;
use crate::icons::zoom_in::*;
use crate::icons::zoom_out::*;
use leptos::html::{Div, Ul};
use leptos::*;

pub fn render_image_list(cx: Scope, data: Vec<String>, auto_scroll: bool) -> HtmlElement<Div> {
    let len = data.len();
    let selected_ref = create_node_ref(cx);
    let node_ref = create_node_ref(cx);
    let (current, set_current) = create_signal(cx, 0);
    let (hidden, set_hidden) = create_signal(cx, true);
    let (zoom, set_zoom) = create_signal(cx, 1.0);

    let prev = move |_| {
        let cur = current.get();
        if cur > 0 {
            set_current.set(cur - 1);
        }
    };

    let next = move |_| {
        let cur = current.get();
        if cur < len - 1 {
            set_current.set(cur + 1);
        }
    };

    let activate = move |index: usize| {
        set_current.set(index);
    };

    let open = move || {
        let parent_el: HtmlElement<Ul> = node_ref.get().unwrap();
        let selected_el: HtmlElement<Div> = selected_ref.get().unwrap();

        if let Ok(Some(img)) = parent_el.query_selector(".image-list__image.active img") {
            if let Ok(cloned) = img.clone_node() {
                let _ = selected_el.append_child(&cloned);
                set_hidden(false);
            }
        }
    };

    let close = move || {
        set_hidden(true);
        set_zoom(1.0);
        let selected_el: HtmlElement<Div> = selected_ref.get().unwrap();
        selected_el.set_inner_html("");
    };

    let make_zoom = create_action(cx, move |fact: &f64| {
        let el: HtmlElement<Div> = selected_ref.get().unwrap();
        if let Ok(Some(img)) = el.query_selector(&format!("img")) {
            let z = zoom.get() * (*fact);
            let _ = img.set_attribute("style", &format!("transform:scale({});", z));
            set_zoom.set(z);
        }
        async move {}
    });

    let zoom_in = move |_| make_zoom.dispatch(1.1);

    let zoom_out = move |_| make_zoom.dispatch(0.9);

    if auto_scroll {
        scroll_to_bottom(node_ref);
    }

    view! {
        cx,
        <div class="image-list">
            <div class="image-list__avatar">
                <ChatGptIcon />
            </div>
            <ul
                class="image-list__container"
                class:multiple=move || { len > 1 }
                node_ref=node_ref
            >
                {
                    move || (len > 1).then(|| view! {
                        cx,
                        <li
                            class="image-list--prev"
                            class:disabled=move || current.get() == 0
                            on:click=prev
                        >
                            <LeftIcon />
                        </li>
                    })
                }
                <For
                    each=move || data.clone().into_iter().enumerate()
                    key=|(index,_)| *index
                    view=move |cx, (index, src)| view! {
                        cx,
                        <li
                            class="image-list__image"
                            class:left=move || { current.get() > index }
                            class:right=move || { current.get() < index }
                            class:active=move || { current.get() == index }
                            on:click=move |_| open()
                        >
                            <img src=src />
                        </li>
                    }
                />
                {
                    move || (len > 1).then(|| view! {
                        cx,
                        <li
                            class="image-list--next"
                            class:disabled=move || current.get() == len - 1
                            on:click=next
                        >
                            <RightIcon />
                        </li>
                    })
                }
                {
                    move || (len > 1).then(|| view! {
                        cx,
                        <li
                            class="image-list--selection"
                        >
                            <For
                                each=move || (0..len).into_iter()
                                key=|v| *v
                                view=move |cx, v| view!{
                                    cx,
                                    <button
                                        class:active=move || current.get() == v
                                        on:click=move |_| activate(v)
                                    >
                                    </button>
                                }
                            />
                        </li>
                    })
                }
            </ul>
            <div
                class="image-list--selected"
                class:hidden=hidden
                node_ref=selected_ref
                on:click=move |_| close()
            >
            </div>
            <div class="image-list__zoom" class:hidden=hidden>
                <button class="btn" on:click=zoom_in><ZoomInIcon /></button>
                <button class="btn" on:click=zoom_out><ZoomOutIcon /></button>
            </div>
        </div>
    }
}

pub fn render_image_list_error(cx: Scope, err: impl IntoView) -> HtmlElement<Div> {
    view! {
        cx,
        <div class="image-list">
            <div class="image-list__avatar">
                <ChatGptIcon />
            </div>
            <ul>
                <li><ImageBrokenIcon /></li>
                <li>{ err }</li>
            </ul>
        </div>
    }
}

#[component]
pub fn ImageLoading(cx: Scope) -> impl IntoView {
    view! {
        cx,
         <div class="image-list">
            <div class="image-list__avatar">
                <ChatGptIcon />
            </div>
            <ul class="image-list--loading">
                <li><ImageEditingIcon /></li>
            </ul>
        </div>
    }
}
