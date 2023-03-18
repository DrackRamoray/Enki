use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct ImgVarCtx {
    pub image: ReadSignal<String>,
    pub set_image: WriteSignal<String>,
    pub n: ReadSignal<usize>,
    pub set_n: WriteSignal<usize>,
    pub size: ReadSignal<usize>,
    pub set_size: WriteSignal<usize>,
    pub fmt: ReadSignal<usize>,
    pub set_fmt: WriteSignal<usize>,
}

pub fn use_img_var_ctx(cx: Scope) -> ImgVarCtx {
    let (image, set_image) = create_signal(cx, String::new());
    let (n, set_n) = create_signal(cx, 1);
    let (size, set_size) = create_signal(cx, 0);
    let (fmt, set_fmt) = create_signal(cx, 1);

    ImgVarCtx {
        image,
        set_image,
        n,
        set_n,
        size,
        set_size,
        fmt,
        set_fmt,
    }
}
