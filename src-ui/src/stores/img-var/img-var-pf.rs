use crate::assists::error::beautify_error;
use crate::stores::img_var_ctx::{use_img_var_ctx, ImgVarCtx};
use enki_shared::image_category::ImageCategory;
use enki_shared::{
    image_format::ImageFormat, image_size::ImageSize, image_variate::ImageVariateParams,
};
use leptos::*;
use tauri_sys::tauri;

#[derive(Debug, Copy, Clone)]
pub struct ImgVarPfCtx {
    pub perform: Resource<i64, Result<Vec<String>, String>>,
}

pub fn use_img_var_pf_ctx(
    cx: Scope,
    topic_id: ReadSignal<i64>,
    on_finished: Action<(), ()>,
) -> ImgVarPfCtx {
    let ctx = use_context::<ImgVarCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_var_ctx(cx));

    let perform = create_resource(
        cx,
        move || (topic_id.get()),
        move |topic_id| async move {
            let params = ImageVariateParams {
                image: ctx.image.get(),
                n: ctx.n.get(),
                size: ImageSize::value_from_index(ctx.size.get()),
                fmt: ImageFormat::value_from_index(ctx.fmt.get()),
                category: ImageCategory::Variate,
                topic_id,
            };

            match tauri::invoke::<_, Vec<String>>("plugin:image_variate|variate_image", &params)
                .await
            {
                Ok(images) => {
                    on_finished.dispatch(());
                    Ok(images)
                }
                Err(err) => {
                    on_finished.dispatch(());
                    Err(beautify_error(err))
                }
            }
        },
    );

    ImgVarPfCtx { perform }
}
