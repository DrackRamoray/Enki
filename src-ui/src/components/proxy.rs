use crate::components::mask::*;
use crate::icons::cancel::*;
use crate::icons::confirm::*;
use crate::icons::proxy::*;
use crate::stores::proxy_ctx::{use_proxy_ctx, PROXY_PROTOCOLS};
use leptos::*;

#[component]
pub fn Proxy(cx: Scope) -> impl IntoView {
    let protocols = PROXY_PROTOCOLS;
    let ctx = use_proxy_ctx(cx);
    let toggle = move |_| ctx.set_hidden.update(|value| *value = !*value);

    view! {
        cx,
        <button class="btn" on:click=toggle>
            <ProxyIcon />
        </button>
        <div class="proxy__popup" class:invisible=ctx.hidden>
            <div class="proxy__radio-group">
                <For
                    each=move || { protocols.into_iter().enumerate() }
                    key=|(index, _)| *index
                    view=move |cx, (_, &item)| view! {
                        cx,
                        <div>
                            <input
                                id=item
                                type="radio"
                                name="protocol"
                                value=item
                                class="hidden peer"
                                prop:checked=move || { ctx.protocol.get() == item }
                                on:change=move |evt| (ctx.set_protocol)(event_target_value(&evt))
                            />
                            <label for=item class="proxy__radio-group__label">
                                {item}
                            </label>
                        </div>
                    }
                />
            </div>
            <div class="w-full">
                <input
                    id="server"
                    class="input w-full"
                    placeholder="server"
                    value=ctx.server
                    on:change=move |evt| (ctx.set_server)(event_target_value(&evt))
                />
            </div>
            <div class="w-full">
                <input
                    id="port"
                    class="input w-full"
                    placeholder="port"
                    value=ctx.port
                    on:change=move |evt| (ctx.set_port)(event_target_value(&evt))
                />
            </div>
            <div class="proxy__operation">
                <button class="btn" on:click=move |_| ctx.save_proxy.dispatch(())>
                    <ConfirmIcon />
                </button>
                <button class="btn" on:click=toggle>
                    <CancelIcon />
                </button>
            </div>
        </div>
        <Mask hidden=ctx.hidden set_hidden=ctx.set_hidden class="mask" />
    }
}
