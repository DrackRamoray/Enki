use leptos::*;

#[component]
pub fn ErrorWarningIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0"></path>
           <path d="M12 9v4"></path>
           <path d="M12 16v.01"></path>
        </svg>
    }
}
