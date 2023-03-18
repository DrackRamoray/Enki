use crate::components::image_list::*;
use crate::components::img_edit_speaker::*;
use crate::stores::img_edit_ctx::{use_img_edit_ctx, ImgEditCtx};
use crate::stores::img_edit_pf_ctx::use_img_edit_pf_ctx;
use leptos::html::Div;
use leptos::*;

pub fn render_img_edit_pf(
    cx: Scope,
    parent_ref: NodeRef<Div>,
    topic_id: i64,
    on_finished: Action<(), ()>,
) {
    let ctx = use_context::<ImgEditCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_edit_ctx(cx));
    let ctx_pf = use_img_edit_pf_ctx(cx, topic_id, on_finished);

    let el = view! {
        cx,
        <div>
            <ImgEditPromptSpeaker
                image=ctx.image.get()
                mask=ctx.mask.get()
                prompt=ctx.prompt.get()
                auto_scroll=true
            />
            {
                move || match ctx_pf.perform.read(cx) {
                    None => view! { cx, <> <ImageLoading /> </> },
                    Some(Ok(images)) => view! { cx, <>{ render_image_list(cx, images, true) }</> },
                    Some(Err(err)) => view! { cx, <>{ render_image_list_error(cx, err) }</> },
                }
            }
        </div>
    };

    let parent_el = parent_ref.get().unwrap();

    let _ = parent_el.append_child(&el);
}
