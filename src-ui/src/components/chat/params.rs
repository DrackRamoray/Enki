use crate::icons::prompt_setting::*;
use crate::stores::chat_params::{use_chat_params_context, ChatParamsContext};
use enki_shared::chat_model::ChatModel;
use leptos::html::Div;
use leptos::*;

#[component]
pub fn ChatParams(cx: Scope) -> impl IntoView {
    let ctx = use_context::<ChatParamsContext>(cx);
    let ctx = ctx.unwrap_or_else(|| use_chat_params_context(cx));
    let instruction_ref = create_node_ref(cx);
    let (hidden, set_hidden) = create_signal(cx, true);
    let toggle = move |_| set_hidden.update(|v| *v = !*v);

    let on_instruction_input = move |_| {
        let div: HtmlElement<Div> = instruction_ref.get().unwrap();
        ctx.set_instruction.set(div.inner_text());
    };

    view! {
        cx,
        <button class="btn" on:click=toggle>
            <PromptSettingIcon />
        </button>
        <div class="chat-params__popup" class:hidden=hidden>
            <div class="params__item">
                <span
                    class="chat-params__placeholder"
                    class:hidden=move || ctx.instruction.get().trim().len() != 0
                >
                    "You are helpful assistant"
                </span>
                <div
                    class="chat-params__editable"
                    contenteditable
                    node_ref=instruction_ref
                    on:input=on_instruction_input
                >
                </div>
            </div>
            <div class="params__item">
                <For
                    each=move || ChatModel::vec().into_iter().enumerate()
                    key=|(index, _)| *index
                    view=move |cx, (index, item)| view! {
                        cx,
                        <div class="params__radio">
                            <label prop:for=item.clone() >
                                <input
                                    prop:id=item.clone()
                                    name="model"
                                    type="radio"
                                    checked=move || ctx.model.get() == index
                                    on:change=move |_| ctx.set_model.set(index)
                                />
                                <span>{ item }</span>
                            </label>
                        </div>
                    }
                />
            </div>
            <div class="params__item params__number">
                <label for="steps-range">
                    <span>"Temperature"</span>
                    <input
                        class="input--sm"
                        type="number"
                        min="0.0"
                        max="2.0"
                        prop:value=ctx.temperature
                        on:change=move |evt| ctx.set_temperature.set(event_target_value(&evt).parse::<f64>().unwrap_or_default())
                    />
                </label>
            </div>
            <div class="params__item params__number">
                <label for="steps-range">
                    <span>"Top P"</span>
                    <input
                        class="input--sm"
                        type="number"
                        min="0.0"
                        max="1.0"
                        prop:value=ctx.top_p
                        on:change=move |evt| ctx.set_top_p.set(event_target_value(&evt).parse::<f64>().unwrap_or_default())
                    />
                </label>
            </div>
            <div class="params__item params__number">
                <label for="steps-range">
                    <span>"n"</span>
                    <input
                        class="input--sm"
                        id="steps-range"
                        type="number"
                        min="1"
                        max="10"
                        prop:value=ctx.n
                        on:change=move |evt| ctx.set_n.set(event_target_value(&evt).parse::<i32>().unwrap_or_default())
                    />
                </label>
            </div>
            <div class="params__item params__number">
                <label for="steps-range">
                    <span>"Max Tokens"</span>
                    <input
                        class="input--sm"
                        type="number"
                        min="1"
                        max="2048"
                        prop:value=ctx.max_tokens
                        on:change=move |evt| ctx.set_max_tokens.set(event_target_value(&evt).parse::<i32>().unwrap_or_default())
                    />
                </label>
            </div>
            <div class="params__item params__number">
                <label for="steps-range">
                    <span>"Presence Penalty"</span>
                    <input
                        class="input--sm"
                        type="number"
                        min="-2.0"
                        max="2.0"
                        prop:value=ctx.presence_penalty
                        on:change=move |evt| ctx.set_presence_penalty.set(event_target_value(&evt).parse::<f64>().unwrap_or_default())
                    />
                </label>
            </div>
            <div class="params__item params__number">
                <label for="steps-range">
                    <span>"Frequency Penalty"</span>
                    <input
                        class="input--sm"
                        type="number"
                        min="-2.0"
                        max="2.0"
                        prop:value=ctx.frequency_penalty
                        on:change=move |evt| ctx.set_frequency_penalty.set(event_target_value(&evt).parse::<f64>().unwrap_or_default())
                    />
                </label>
            </div>
            <div class="params__item">
                <label class="params__checkbox" for="provide-previous-message">
                    <input
                        id="provide-previous-message"
                        type="checkbox"
                        prop:value=ctx.provide_previous_messages
                        on:change=move |_| ctx.set_provide_previous_messages.update(|v| *v = !*v)
                    />
                    <span>"Provide Previous Message"</span>
                </label>
            </div>
        </div>
    }
}
