use leptos::*;

#[derive(Debug, Copy, Clone)]
pub struct ImgGenCtx {
    pub prompt: ReadSignal<String>,
    pub set_prompt: WriteSignal<String>,
    pub n: ReadSignal<usize>,
    pub set_n: WriteSignal<usize>,
    pub size: ReadSignal<usize>,
    pub set_size: WriteSignal<usize>,
    pub fmt: ReadSignal<usize>,
    pub set_fmt: WriteSignal<usize>,
}

pub fn use_img_gen_ctx(cx: Scope) -> ImgGenCtx {
    let (prompt, set_prompt) = create_signal(cx, String::new());
    let (n, set_n) = create_signal(cx, 1);
    let (size, set_size) = create_signal(cx, 0);
    let (fmt, set_fmt) = create_signal(cx, 1);

    ImgGenCtx {
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
