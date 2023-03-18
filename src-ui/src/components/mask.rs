use leptos::*;

#[component]
pub fn Mask(
    cx: Scope,
    hidden: ReadSignal<bool>,
    set_hidden: WriteSignal<bool>,
    class: &'static str,
) -> impl IntoView {
    let visible = Signal::derive(cx, move || !hidden.get());
    let on_click = move |_| set_hidden.update(|value| *value = !*value);

    view! {
        cx,
        <>
            {
                move || {
                    visible.get().then(move || {
                        view! {
                            cx,
                            <div class=class on:click=on_click></div>
                        }
                    })
                }
            }
        </>
    }
}
