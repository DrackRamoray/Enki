use leptos::*;

#[derive(Copy, Clone)]
pub struct AudioTranscriptCtx {
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
    pub language: ReadSignal<String>,
    pub set_language: WriteSignal<String>,
}

pub fn use_audio_transcript_ctx(cx: Scope) -> AudioTranscriptCtx {
    let (file, set_file) = create_signal(cx, String::new());
    let (model, set_model) = create_signal(cx, 0);
    let (prompt, set_prompt) = create_signal(cx, String::new());
    let (fmt, set_fmt) = create_signal(cx, 0);
    let (temperature, set_temperature) = create_signal(cx, 0.0);
    let (language, set_language) = create_signal(cx, String::new());

    AudioTranscriptCtx {
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
        language,
        set_language,
    }
}
