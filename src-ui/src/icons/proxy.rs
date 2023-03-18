use leptos::*;

#[component]
pub fn ProxyIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M20 4v8"></path>
           <path d="M16 4.5v7"></path>
           <path d="M12 5v16"></path>
           <path d="M8 5.5v5"></path>
           <path d="M4 6v4"></path>
           <path d="M20 8h-16"></path>
        </svg>
    }
}
