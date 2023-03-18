use crate::components::image_params::*;
use crate::icons::prompt_setting::*;
use crate::stores::img_var_ctx::{use_img_var_ctx, ImgVarCtx};
use leptos::*;

#[component]
pub fn ImgVarParams(cx: Scope) -> impl IntoView {
    let ctx = use_context::<ImgVarCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_var_ctx(cx));

    let (hidden, set_hidden) = create_signal(cx, true);
    let toggle = move |_| set_hidden.update(|v| *v = !*v);

    view! {
        cx,
        <button class="btn" on:click=toggle>
            <PromptSettingIcon />
        </button>
        <div class="image-params__popup" hidden=hidden>
            <ImageCommonParams
                n=ctx.n
                set_n=ctx.set_n
                size=ctx.size
                set_size=ctx.set_size
                fmt=ctx.fmt
                set_fmt=ctx.set_fmt
            />
        </div>
    }
}
