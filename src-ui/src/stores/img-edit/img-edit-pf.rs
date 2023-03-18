use crate::assists::error::beautify_error;
use crate::stores::img_edit_ctx::{use_img_edit_ctx, ImgEditCtx};
use enki_shared::image_category::ImageCategory;
use enki_shared::{image_edit::ImageEditParams, image_format::ImageFormat, image_size::ImageSize};
use leptos::*;
use tauri_sys::tauri;

#[derive(Debug, Copy, Clone)]
pub struct ImgEditPfCtx {
    pub perform: Resource<(), Result<Vec<String>, String>>,
}

pub fn use_img_edit_pf_ctx(cx: Scope, topic_id: i64, on_finished: Action<(), ()>) -> ImgEditPfCtx {
    let ctx = use_context::<ImgEditCtx>(cx);
    let ctx = ctx.unwrap_or_else(|| use_img_edit_ctx(cx));

    let perform = create_resource(
        cx,
        move || (),
        move |_| async move {
            let params = ImageEditParams {
                image: ctx.image.get(),
                mask: if ctx.mask.get().is_empty() {
                    None
                } else {
                    Some(ctx.mask.get())
                },
                prompt: ctx.prompt.get(),
                n: ctx.n.get(),
                size: ImageSize::value_from_index(ctx.size.get()),
                fmt: ImageFormat::value_from_index(ctx.fmt.get()),
                category: ImageCategory::Edit,
                topic_id,
            };

            match tauri::invoke::<_, Vec<String>>("plugin:image_edit|edit_image", &params).await {
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

    ImgEditPfCtx { perform }
}
