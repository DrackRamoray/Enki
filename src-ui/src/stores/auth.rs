use crate::assists::error::beautify_error;
use crate::assists::notify::{notify, NotifyType};
use enki_shared::auth::AuthData;
use leptos::*;
use tauri_sys::{clipboard, tauri};

pub struct AuthCtx {
    pub hidden: ReadSignal<bool>,
    pub set_hidden: WriteSignal<bool>,
    pub token: ReadSignal<String>,
    pub set_token: WriteSignal<String>,
    pub load_token: Resource<(), Option<String>>,
    pub save_token: Action<(), ()>,
    pub copy_token: Action<(), ()>,
}

pub fn use_auth_ctx(cx: Scope) -> AuthCtx {
    let (hidden, set_hidden) = create_signal(cx, true);
    let (token, set_token) = create_signal(cx, String::new());

    let load_token = create_resource(
        cx,
        move || (),
        move |_| async move {
            match tauri::invoke::<_, AuthData>("plugin:auth|load_auth_data", &()).await {
                Err(err) => {
                    error!("load auth data failed: {:?}", err);
                    notify(cx, NotifyType::Error, beautify_error(err));
                    None
                }
                Ok(auth) => {
                    if auth.token.len() == 0 {
                        None
                    } else {
                        Some(auth.token)
                    }
                }
            }
        },
    );

    let save_token = create_action(cx, move |_| async move {
        let token = token.get().trim().to_string();

        if token.is_empty() {
            notify(cx, NotifyType::Warn, "token can not be empty");
            return;
        }

        let auth = AuthData { token };

        if let Err(err) = tauri::invoke::<_, ()>("plugin:auth|save_auth_data", &auth).await {
            error!("save auth data failed: {:?}", err);
            notify(cx, NotifyType::Error, beautify_error(err));
        }

        load_token.refetch();
    });

    let copy_token = create_action(cx, move |_| async move {
        if let Some(Some(token)) = load_token.read(cx) {
            if let Err(err) = clipboard::write_text(&token).await {
                notify(cx, NotifyType::Error, beautify_error(err.clone()));
                error!("copy token failed: {:?}", err);
            }
        } else {
            notify(cx, NotifyType::Warn, "copy failed.");
        }
    });

    AuthCtx {
        hidden,
        set_hidden,
        token,
        set_token,
        load_token,
        save_token,
        copy_token,
    }
}
