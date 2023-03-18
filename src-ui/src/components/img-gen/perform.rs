use crate::components::image_list::*;
use crate::components::img_gen_speaker::*;
use crate::stores::img_gen_pf_ctx::use_img_gen_pf_ctx;
use leptos::html::Div;
use leptos::*;

pub fn render_img_gen_pf(
    cx: Scope,
    parent_ref: NodeRef<Div>,
    topic_id: i64,
    on_finished: Action<(), ()>,
) {
    let ctx = use_img_gen_pf_ctx(cx, topic_id, on_finished);

    let el = view! {
        cx,
        <div>
            <ImgGenSpeaker />
            {
                move || match ctx.perform.read(cx) {
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
