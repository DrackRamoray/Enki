use leptos::*;

#[component]
pub fn ErrorIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M4 8v-2a2 2 0 0 1 2 -2h2"></path>
           <path d="M4 16v2a2 2 0 0 0 2 2h2"></path>
           <path d="M16 4h2a2 2 0 0 1 2 2v2"></path>
           <path d="M16 20h2a2 2 0 0 0 2 -2v-2"></path>
           <path d="M9 10h.01"></path>
           <path d="M15 10h.01"></path>
           <path d="M9.5 15.05a3.5 3.5 0 0 1 5 0"></path>
        </svg>
    }
}
