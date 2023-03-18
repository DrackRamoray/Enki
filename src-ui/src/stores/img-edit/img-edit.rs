use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct ImgEditCtx {
    pub image: ReadSignal<String>,
    pub set_image: WriteSignal<String>,
    pub mask: ReadSignal<String>,
    pub set_mask: WriteSignal<String>,
    pub prompt: ReadSignal<String>,
    pub set_prompt: WriteSignal<String>,
    pub n: ReadSignal<usize>,
    pub set_n: WriteSignal<usize>,
    pub size: ReadSignal<usize>,
    pub set_size: WriteSignal<usize>,
    pub fmt: ReadSignal<usize>,
    pub set_fmt: WriteSignal<usize>,
}

pub fn use_img_edit_ctx(cx: Scope) -> ImgEditCtx {
    let (image, set_image) = create_signal(cx, String::new());
    let (mask, set_mask) = create_signal(cx, String::new());
    let (prompt, set_prompt) = create_signal(cx, String::new());
    let (n, set_n) = create_signal(cx, 1);
    let (size, set_size) = create_signal(cx, 0);
    let (fmt, set_fmt) = create_signal(cx, 1);

    ImgEditCtx {
        image,
        set_image,
        mask,
        set_mask,
        prompt,
        set_prompt,
        n,
        set_n,
        size,
        set_size,
        fmt,
        set_fmt,
    }
}
