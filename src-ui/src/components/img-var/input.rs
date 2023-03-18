use crate::assists::constants::SUPPORTED_IMG;
use crate::assists::notify::{notify, NotifyType};
use crate::components::file_upload::*;
use crate::components::img_var_params::*;
use crate::components::img_var_perform::render_img_var_pf;
use crate::icons::prompt_send::*;
use crate::stores::img_var_ctx::{use_img_var_ctx, ImgVarCtx};
use leptos::html::Div;
use leptos::*;
use std::time::Duration;

#[component]
pub fn ImgVarInput(
    cx: Scope,
    sending: ReadSignal<bool>,
    set_sending: WriteSignal<bool>,
    topic_id: ReadSignal<i64>,
    parent_ref: NodeRef<Div>,
) -> impl IntoView {
    let ctx = use_context::<ImgVarCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_var_ctx(cx));

    let on_finished = create_action(cx, move |_: &()| {
        set_sending.set(false);

        async move {}
    });

    let send = create_action(cx, move |_: &()| async move {
        if ctx.image.get().is_empty() {
            set_sending(false);
            notify(cx, NotifyType::Warn, "image is required.");
            return;
        }

        render_img_var_pf(cx, parent_ref, topic_id, on_finished);
        set_timeout(
            move || {
                (ctx.set_image)(String::new());
            },
            Duration::from_millis(20),
        );
    });

    view! {
        cx,
        <FileUpload
            file_path=ctx.image
            set_file_path=ctx.set_image
            supported_extensions=SUPPORTED_IMG
        >
            <span>"image"</span>
        </FileUpload>
        <div>
            <button
                class="btn"
                prop:disabled=sending
                on:click=move |_| send.dispatch(())
            >
                <PromptSendIcon />
            </button>
            <ImgVarParams />
        </div>
    }
}
