use crate::assists::scroll::scroll_to_bottom;
use crate::components::prompt_speaker::render_prompt_speaker;
use crate::stores::img_gen_ctx::{use_img_gen_ctx, ImgGenCtx};
use leptos::*;

#[component]
pub fn ImgGenSpeaker(cx: Scope) -> impl IntoView {
    let node_ref = create_node_ref(cx);
    let ctx = use_context::<ImgGenCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_gen_ctx(cx));

    scroll_to_bottom(node_ref);

    render_prompt_speaker(cx, node_ref, ctx.prompt.get())
}
