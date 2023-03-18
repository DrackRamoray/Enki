use crate::assists::constants::SUPPORTED_IMG;
use crate::assists::notify::{notify, NotifyType};
use crate::components::file_upload::*;
use crate::components::img_edit_params::*;
use crate::components::img_edit_perform::render_img_edit_pf;
use crate::components::prompt::*;
use crate::stores::img_edit_ctx::{use_img_edit_ctx, ImgEditCtx};
use leptos::html::Div;
use leptos::*;
use std::time::Duration;

#[component]
pub fn ImgEditInput(
    cx: Scope,
    sending: ReadSignal<bool>,
    set_sending: WriteSignal<bool>,
    topic_id: ReadSignal<i64>,
    parent_ref: NodeRef<Div>,
) -> impl IntoView {
    let ctx = use_context::<ImgEditCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_edit_ctx(cx));

    let on_finished = create_action(cx, move |_: &()| {
        set_sending.set(false);

        async move {}
    });

    let send = create_action(cx, move |_: &()| async move {
        if ctx.image.get().is_empty() {
            set_sending(false);
            notify(cx, NotifyType::Warn, "image is required.");
        }

        render_img_edit_pf(cx, parent_ref, topic_id.get(), on_finished);
        set_timeout(
            move || {
                (ctx.set_image)(String::new());
                (ctx.set_mask)(String::new());
            },
            Duration::from_millis(20),
        );
    });

    view! {
        cx,
        <>
            <div class="image-edit-input__uploader">
                <FileUpload
                    file_path=ctx.image
                    set_file_path=ctx.set_image
                    supported_extensions=SUPPORTED_IMG
                >
                    <span>"image"</span>
                </FileUpload>
                <FileUpload
                    file_path=ctx.mask
                    set_file_path=ctx.set_mask
                    supported_extensions=SUPPORTED_IMG
                >
                    <span>"mask"</span>
                </FileUpload>
            </div>
            <div class="image-edit-input__wrap">
                <Prompt
                    sending=sending
                    set_sending=set_sending
                    message_box=ctx.prompt
                    set_message_box=ctx.set_prompt
                    send=send
                />
                <ImgEditParams />
            </div>
        </>
    }
}
