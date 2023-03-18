use crate::assists::error::beautify_error;
use crate::components::error_warning::*;
use crate::components::img_gen_params::*;
use crate::components::img_gen_perform::*;
use crate::components::img_gen_record::*;
use crate::components::prompt::*;
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use crate::stores::img_gen_ctx::use_img_gen_ctx;
use enki_shared::image_generate_record::{ImageGenerateRecord, ImageGenerateRecordParams};
use enki_shared::topic_category::TopicCategory;
use leptos::html::Div;
use leptos::*;
use tauri_sys::tauri;

#[component]
pub fn ImgGen(cx: Scope) -> impl IntoView {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    if app_ctx.active_topic_ctg.get() != TopicCategory::ImageGenerate.to_index() {
        return view! { cx, <div></div> };
    }

    let parent_ref = create_node_ref::<Div>(cx);
    let ctx = use_img_gen_ctx(cx);

    provide_context(cx, ctx);

    let fetcher = create_resource(
        cx,
        move || (),
        move |_| async move {
            let params = ImageGenerateRecordParams {
                id: app_ctx.active_topic_id.get(),
            };

            match tauri::invoke::<_, Vec<ImageGenerateRecord>>(
                "plugin:image_generate|get_image_generate_record",
                &params,
            )
            .await
            {
                Ok(data) => Ok(data),
                Err(err) => {
                    error!("fetch image generated failed: {:?}", err);
                    Err(beautify_error(err))
                }
            }
        },
    );

    let on_finished = create_action(cx, move |_: &()| {
        app_ctx.set_sending.set(false);

        async move {}
    });

    let send = create_action(cx, move |_: &()| {
        render_img_gen_pf(cx, parent_ref, app_ctx.active_topic_id.get(), on_finished);
        async move {}
    });

    view! {
        cx,
        <div class="view view--sm">
            <div class="view__body" node_ref=parent_ref>
                {
                    move || fetcher.read(cx).map(|result| match result {
                        Ok(data) => view! { cx, <><ImgGenRecord data=data /></> },
                        Err(err) => view! { cx, <> <ErrorWarning err=err /> </> },
                    })
                }
            </div>
            <div class="view__footer view__footer--flex">
                <Prompt
                    sending=app_ctx.sending
                    set_sending=app_ctx.set_sending
                    message_box=ctx.prompt
                    set_message_box=ctx.set_prompt
                    send=send
                />
                <ImgGenParams />
            </div>
        </div>
    }
}
