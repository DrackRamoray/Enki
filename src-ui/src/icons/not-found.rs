use leptos::*;

#[component]
pub fn NotFoundIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="180" height="180" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M3 7v4a1 1 0 0 0 1 1h3"></path>
           <path d="M7 7v10"></path>
           <path d="M10 8v8a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-8a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1z"></path>
           <path d="M17 7v4a1 1 0 0 0 1 1h3"></path>
           <path d="M21 7v10"></path>
        </svg>
    }
}
