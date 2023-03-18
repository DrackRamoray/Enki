use crate::assists::error::beautify_error;
use crate::stores::img_gen_ctx::{use_img_gen_ctx, ImgGenCtx};
use enki_shared::image_category::ImageCategory;
use enki_shared::image_format::ImageFormat;
use enki_shared::image_generate::ImageGenerateParams;
use enki_shared::image_size::ImageSize;
use leptos::*;
use tauri_sys::tauri;

#[derive(Debug, Copy, Clone)]
pub struct ImgGenPfCtx {
    pub perform: Resource<(), Result<Vec<String>, String>>,
}

pub fn use_img_gen_pf_ctx(cx: Scope, topic_id: i64, on_finished: Action<(), ()>) -> ImgGenPfCtx {
    let ctx = use_context::<ImgGenCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_gen_ctx(cx));

    let perform = create_resource(
        cx,
        move || (),
        move |_| async move {
            let params = ImageGenerateParams {
                prompt: ctx.prompt.get(),
                n: ctx.n.get(),
                size: ImageSize::value_from_index(ctx.size.get()),
                fmt: ImageFormat::value_from_index(ctx.fmt.get()),
                category: ImageCategory::Generate,
                topic_id,
            };

            match tauri::invoke::<_, Vec<String>>("plugin:image_generate|generate_image", &params)
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

    ImgGenPfCtx { perform }
}
