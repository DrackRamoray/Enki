use leptos::*;

#[component]
pub fn ZoomOutIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0"></path>
           <path d="M7 10l6 0"></path>
           <path d="M21 21l-6 -6"></path>
        </svg>
    }
}
