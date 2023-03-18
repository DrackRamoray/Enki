use crate::icons::prompt_setting::*;
use enki_shared::audio_format::AudioFormat;
use leptos::*;

#[component]
pub fn AudioParams(
    cx: Scope,
    temperature: ReadSignal<f64>,
    set_temperature: WriteSignal<f64>,
    fmt: ReadSignal<usize>,
    set_fmt: WriteSignal<usize>,
    children: Children,
) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(cx, true);
    let toggle = move |_| {
        set_hidden.update(|v| *v = !*v);
    };

    view! {
        cx,
        <button class="btn" on:click=toggle>
            <PromptSettingIcon />
        </button>
        <div class="audio-params__popup" class:hidden=hidden>
            <div class="audio-params__input">
                <span>"temperature"</span>
                <label>
                    <input
                        class="input--sm"
                        type="number"
                        min="0.0"
                        max="1.0"
                        prop:value=temperature
                        on:change=move |evt| set_temperature.set(event_target_value(&evt).parse::<f64>().unwrap())
                    />
                </label>
            </div>
            <div>
                <div class="params__number__title mb-2">"size"</div>
                <For
                    each=move || AudioFormat::vec().into_iter().enumerate()
                    key=|(index, _)| *index
                    view=move |cx,(index, item)| view!{
                        cx,
                        <label class="params__radio">
                            <input
                                type="radio"
                                name="size"
                                checked=move || fmt.get() == index
                                on:change=move |_| set_fmt.set(index)
                            />
                            <span>{ item }</span>
                        </label>
                    }
                />
            </div>
            <div>{ children(cx) }</div>
        </div>
    }
}
