use crate::icons::error_warning::*;
use leptos::*;

#[component]
pub fn ErrorWarning(cx: Scope, err: String) -> impl IntoView {
    view! {
        cx,
        <div class="w-full h-full flex flex-col justify-content items-center dark:text-dark-quinary">
            <div><ErrorWarningIcon /></div>
            <div>{ err }</div>
        </div>
    }
}
