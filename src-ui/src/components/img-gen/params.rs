use crate::components::image_params::*;
use crate::icons::prompt_setting::*;
use crate::stores::img_gen_ctx::{use_img_gen_ctx, ImgGenCtx};
use leptos::*;

#[component]
pub fn ImgGenParams(cx: Scope) -> impl IntoView {
    let ctx = use_context::<ImgGenCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_gen_ctx(cx));

    let (hidden, set_hidden) = create_signal(cx, true);
    let toggle = move |_| set_hidden.update(|v| *v = !*v);

    view! {
        cx,
        <button class="btn" on:click=toggle>
            <PromptSettingIcon />
        </button>
        <div class="image-params__popup" class:hidden=hidden>
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
