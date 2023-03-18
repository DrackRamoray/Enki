use crate::components::mask::*;
use crate::icons::auth::*;
use crate::icons::copy::*;
use crate::icons::save::*;
use crate::stores::auth_ctx::use_auth_ctx;
use leptos::*;
use web_sys::KeyboardEvent;

#[component]
pub fn Auth(cx: Scope) -> impl IntoView {
    let ctx = use_auth_ctx(cx);
    let toggle = move |_| ctx.set_hidden.update(|value| *value = !*value);
    let on_keyup = move |evt: KeyboardEvent| {
        if evt.code() == "Enter" {
            ctx.save_token.dispatch(());
        }
    };
    let show_token = move |t: String| {
        if t.len() > 15 {
            t[0..9].to_string() + "..." + &t[t.len() - 6..]
        } else {
            t
        }
    };

    view! {
        cx,
        <button class="btn" on:click=toggle>
            <AuthIcon />
        </button>
        <div class="auth-popup" class:invisible=ctx.hidden>
            {
                move || ctx.load_token.read(cx).map(|t| match t {
                    None => view!{ cx, <div></div> },
                    Some(t) => view! {
                        cx,
                        <div class="auth-popup__wrapper mb-6">
                            <input
                                class="input"
                                readonly
                                prop:value=move || show_token(t.clone())
                            />
                            <button class="auth-popup__btn btn" on:click=move |_| ctx.copy_token.dispatch(()) >
                                <CopyIcon />
                            </button>
                        </div>
                    }
                })
            }
            <div class="auth-popup__wrapper">
                <input
                    class="input"
                    placeholder="token"
                    autofocus
                    prop:value=ctx.token
                    on:keyup=on_keyup
                    on:input=move |evt| (ctx.set_token)(event_target_value(&evt))
                />
                <button class="auth-popup__btn btn" on:click=move |_| ctx.save_token.dispatch(())>
                    <SaveIcon />
                </button>
            </div>
        </div>
        <Mask hidden=ctx.hidden set_hidden=ctx.set_hidden class="mask" />
    }
}
