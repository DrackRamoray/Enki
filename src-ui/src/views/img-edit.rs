use crate::assists::error::beautify_error;
use crate::components::error_warning::*;
use crate::components::img_edit_input::*;
use crate::components::img_edit_record::*;
use crate::stores::app_ctx::{use_app_ctx, AppCtx};
use crate::stores::img_edit_ctx::use_img_edit_ctx;
use enki_shared::image_edit_record::{ImageEditRecord, ImageEditRecordParams};
use enki_shared::topic_category::TopicCategory;
use leptos::{html::Div, *};
use tauri_sys::tauri;

#[component]
pub fn ImgEdit(cx: Scope) -> impl IntoView {
    let app_ctx = use_context::<AppCtx>(cx);
    let app_ctx = app_ctx.unwrap_or_else(|| use_app_ctx(cx));

    if app_ctx.active_topic_ctg.get() != TopicCategory::ImageEdit.to_index() {
        return view! { cx, <div></div> };
    }

    let parent_ref = create_node_ref::<Div>(cx);
    let ctx = use_img_edit_ctx(cx);

    provide_context(cx, ctx);

    let fetcher = create_resource(
        cx,
        move || (),
        move |_| async move {
            let params = ImageEditRecordParams {
                id: app_ctx.active_topic_id.get(),
            };

            match tauri::invoke::<_, Vec<ImageEditRecord>>(
                "plugin:image_edit|get_image_edit_record",
                &params,
            )
            .await
            {
                Ok(data) => Ok(data),
                Err(err) => Err(beautify_error(err)),
            }
        },
    );

    view! {
        cx,
        <div class="view view--md">
            <div class="view__body" node_ref=parent_ref>
                {
                    move || fetcher.read(cx).map(|result| match result {
                        Ok(data) => view!{ cx, <><ImgEditRecord data=data /></> },
                        Err(err) => view!{ cx, <><ErrorWarning err=err /></> }
                    })
                }
            </div>
            <div class="view__footer">
                <ImgEditInput
                    sending=app_ctx.sending
                    set_sending=app_ctx.set_sending
                    topic_id=app_ctx.active_topic_id
                    parent_ref=parent_ref
                />
            </div>
        </div>
    }
}
