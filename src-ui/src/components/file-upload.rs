use crate::assists::notify::*;
use crate::icons::cancel::*;
use crate::icons::upload::*;
use enki_shared::convert_src::convert_src;
use leptos::*;
use tauri_sys::dialog;

#[component]
pub fn FileUpload(
    cx: Scope,
    children: Children,
    file_path: ReadSignal<String>,
    set_file_path: WriteSignal<String>,
    supported_extensions: &'static [&'static str],
) -> impl IntoView {
    let (preview, set_preview) = create_signal(cx, String::new());
    let (hidden, set_hidden) = create_signal(cx, true);

    let upload_file = create_action(cx, move |_| async move {
        match dialog::FileDialogBuilder::new()
            .add_filter("image", supported_extensions)
            .pick_file()
            .await
        {
            Ok(Some(file_path)) => {
                set_file_path(file_path.to_string_lossy().to_string());
            }
            Ok(None) => notify(cx, NotifyType::Warn, "upload canceled"),
            Err(err) => notify(cx, NotifyType::Error, format!("{:?}", err)),
        }
    });

    let remove_file = move |_| {
        set_file_path(String::new());
    };

    let show_file_path = move || {
        let path = file_path.get();
        let len = path.len();

        if len <= 30 {
            path
        } else {
            format!("{}...{}", &path[0..10], &path[len - 20..])
        }
    };

    let show_preview = move |_| {
        set_preview.set(file_path.get());
        set_hidden.set(false);
    };

    let close_preview = move |_| {
        set_preview.set(String::new());
        set_hidden.set(true);
    };

    view! {
        cx,
        <div class="file-upload">
            <button
                class="file-upload__btn"
                on:click=move |_| upload_file.dispatch(())
            >
                <UploadIcon />
            </button>
            <span
                class="file-upload__label"
                class:hidden=move || !file_path.get().is_empty()
                on:click=move |_| upload_file.dispatch(())
            >
                { children(cx) }
            </span>
            <span
                class="file-upload__name"
                class:hidden=move || file_path.get().is_empty()
                on:click=show_preview
            >
                {
                    move || view!{cx, <>{ show_file_path() }</> }
                }
            </span>
            <button
                class:hidden=move || file_path.get().is_empty()
                on:click=remove_file
            >
                <CancelIcon />
            </button>
        </div>
        <div
            class="file-upload__popup"
            class:hidden=hidden
            on:click=close_preview
        >
            <img prop:src=move || convert_src(preview.get().as_str()) />
        </div>
    }
}
