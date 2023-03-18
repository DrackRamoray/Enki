use leptos::*;

#[derive(Copy, Clone)]
pub struct AudioTranslateCtx {
    pub file: ReadSignal<String>,
    pub set_file: WriteSignal<String>,
    pub model: ReadSignal<usize>,
    pub set_model: WriteSignal<usize>,
    pub prompt: ReadSignal<String>,
    pub set_prompt: WriteSignal<String>,
    pub fmt: ReadSignal<usize>,
    pub set_fmt: WriteSignal<usize>,
    pub temperature: ReadSignal<f64>,
    pub set_temperature: WriteSignal<f64>,
}

pub fn use_audio_translate_ctx(cx: Scope) -> AudioTranslateCtx {
    let (file, set_file) = create_signal(cx, String::new());
    let (model, set_model) = create_signal(cx, 0);
    let (prompt, set_prompt) = create_signal(cx, String::new());
    let (fmt, set_fmt) = create_signal(cx, 0);
    let (temperature, set_temperature) = create_signal(cx, 0.0);

    AudioTranslateCtx {
        file,
        set_file,
        model,
        set_model,
        prompt,
        set_prompt,
        fmt,
        set_fmt,
        temperature,
        set_temperature,
    }
}
