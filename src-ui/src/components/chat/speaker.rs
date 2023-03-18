use crate::assists::scroll::scroll_to_bottom;
use crate::components::prompt_speaker::*;
use crate::stores::chat_params::{use_chat_params_context, ChatParamsContext};
use leptos::*;

#[component]
pub fn SpeakerMessage(cx: Scope) -> impl IntoView {
    let node_ref = create_node_ref(cx);
    let ctx = use_context::<ChatParamsContext>(cx);
    let ctx = ctx.unwrap_or_else(|| use_chat_params_context(cx));

    scroll_to_bottom(node_ref);

    render_prompt_speaker(cx, node_ref, ctx.message.get())
}
