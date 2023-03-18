use crate::icons::empty::*;
use leptos::*;
use std::time::Duration;

#[component]
pub fn AutoComplete(
    cx: Scope,
    text: ReadSignal<String>,
    set_text: WriteSignal<String>,
    data: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    let (display_text, set_display_text) = create_signal(cx, String::new());
    let (hidden, set_hidden) = create_signal(cx, true);
    let (suggestions, set_suggestions) = create_signal(
        cx,
        data.iter()
            .take(10)
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect::<Vec<(String, String)>>(),
    );

    let update_suggestions = create_action(cx, move |_: &()| async move {
        let v = display_text.get();

        let tmp = data
            .iter()
            .filter(|(name, _)| name.to_lowercase().contains(&v.to_lowercase()))
            .take(10)
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect::<Vec<(String, String)>>();

        set_suggestions.set(tmp);

        if let Some((_, value)) = data
            .iter()
            .find(|(name, _)| name.to_lowercase() == v.to_lowercase())
        {
            set_text.set(value.to_string());
        }
    });

    let on_focus = move |_| {
        set_hidden.set(false);
    };

    let on_change = move |evt| {
        set_display_text.set(event_target_value(&evt));

        request_animation_frame(move || update_suggestions.dispatch(()));
    };

    let on_select = move |index: usize| {
        let (name, value) = suggestions.get()[index].clone();
        set_display_text.set(name);
        set_text.set(value);
        set_hidden.set(true);
    };

    let on_blur = move |_| {
        set_timeout(
            move || {
                let display_text = display_text.get();

                if data
                    .iter()
                    .all(|(name, _)| name.to_lowercase() != display_text.to_lowercase())
                {
                    set_display_text.set(String::new());
                    set_text.set(String::new());
                }

                set_hidden.set(true);
            },
            Duration::from_millis(300),
        );
    };

    view! {
        cx,
        <div class="relative">
             <input
                class="input"
                prop:value=display_text
                on:focus=on_focus
                on:input=on_change
                on:blur=on_blur
            />
            <div class="auto-complete__popup" class:hidden=hidden >
                <ul>
                    {
                        move || {
                            let list = suggestions.get();

                            match list.is_empty() {
                                true => view! { cx,  <><li class="auto-complete__empty"><EmptyIcon /></li></> },
                                false => view! {
                                    cx,
                                    <>
                                        <For
                                            each=move || list.clone().into_iter().enumerate()
                                            key=|(index, _)| *index
                                            view=move |cx, (index, (name, item))| view!{
                                                cx,
                                                <li
                                                    class="dropdown__item"
                                                    class:selected=move || text.get() == item
                                                    on:click=move |_| on_select(index)
                                                >
                                                    { name }
                                               </li>
                                            }
                                        />
                                    </>
                                },
                            }
                        }
                    }
                </ul>
            </div>
        </div>
    }
}
