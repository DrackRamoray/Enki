use leptos::*;

#[component]
pub fn Avatar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <button class="avatar__btn">
            <img src="/public/avatar.png" />
        </button>
        <div class="absolute">
        </div>
    }
}
