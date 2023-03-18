use leptos::*;
use std::time;

#[derive(PartialEq, Clone, Copy)]
pub enum NotifyType {
    Info,
    Warn,
    Success,
    Error,
}

pub fn notify(cx: Scope, notify_type: NotifyType, message: impl IntoView) {
    let doc = document();
    let body = doc.body().unwrap();

    let container = view! {
        cx,
        <div
            class="notify"
            class:info=move || notify_type == NotifyType::Info
            class:warn=move || notify_type == NotifyType::Warn
            class:success=move || notify_type == NotifyType::Success
            class:error=move || notify_type == NotifyType::Error
        >
            { message }
        </div>
    };

    let _ = body.append_with_node_1(&container);

    set_timeout(
        move || {
            container.remove();
        },
        time::Duration::from_millis(3000),
    )
}
