use enki_shared::image_format::ImageFormat;
use enki_shared::image_size::ImageSize;
use leptos::*;

#[component]
pub fn ImageCommonParams(
    cx: Scope,
    n: ReadSignal<usize>,
    set_n: WriteSignal<usize>,
    size: ReadSignal<usize>,
    set_size: WriteSignal<usize>,
    fmt: ReadSignal<usize>,
    set_fmt: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        cx,
        <div class="params__number">
            <label>
                <span>"n"</span>
                <input
                    class="input--sm"
                    type="number"
                    min="1" max="10"
                    prop:value=n
                    on:change=move |evt| set_n.set(event_target_value(&evt).parse::<usize>().unwrap())
                />
            </label>
        </div>
        <div>
            <div class="params__radio__title">"size"</div>
            <For
                each=move || ImageSize::vec().into_iter().enumerate()
                key=|(index, _)| *index
                view=move |cx,(index, item)| view!{
                    cx,
                    <label class="params__radio">
                        <input
                            type="radio"
                            name="size"
                            checked=move || size.get() == index
                            on:change=move |_| set_size.set(index)
                        />
                        <span>{ item }</span>
                    </label>
                }
            />
        </div>
        <div>
            <div class="params__radio__title">"format"</div>
            <For
                each=move || ImageFormat::vec().into_iter().enumerate()
                key=|(index, _)| *index
                view=move |cx,(index, item)| view!{
                    cx,
                    <label class="params__radio">
                        <input
                            type="radio"
                            name="fmt"
                            checked=move || fmt.get() == index
                            on:change=move |_| set_fmt.set(index)
                        />
                        <span>{ item }</span>
                    </label>
                }
            />
        </div>
    }
}
