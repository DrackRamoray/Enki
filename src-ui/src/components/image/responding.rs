use crate::icons::chatgpt::*;
use leptos::*;

#[component]
pub fn ImageResponding(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="animate-pulse">
            <div class="">
                <ChatGptIcon />
            </div>
            <div class=""></div>
        </div>
    }
}
