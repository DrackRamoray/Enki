use leptos::*;

#[component]
pub fn Dropdown<F, T>(
    cx: Scope,
    children: Children,
    data: Vec<T>,
    selected: ReadSignal<i32>,
    on_select: F,
) -> impl IntoView
where
    F: Fn(usize) + Copy + 'static,
    T: IntoView + Clone + 'static,
{
    let (hidden, set_hidden) = create_signal(cx, true);
    let toggle = move |_| set_hidden.update(|value| *value = !*value);
    let select = move |index: usize| {
        set_hidden(true);
        on_select(index);
    };

    view! {
        cx,
        <div class="dropdown__trigger" on:click=toggle>
            { children(cx) }
        </div>
        <div
            class="dropdown__popup"
            class:invisible=hidden
        >
            <ul>
                <For
                    each=move || { data.clone().into_iter().enumerate() }
                    key=|(index, _)| *index
                    view=move |cx, item| view! {
                        cx,
                        <li
                            class="dropdown__item"
                            class:selected=move || selected.get() == item.0 as i32
                            on:click=move |_| select(item.0)
                        >
                            { item.1 }
                        </li>
                    }
                />
                <li class="dropdown__popup__item__bg"></li>
            </ul>
        </div>
    }
}
