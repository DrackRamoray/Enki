use leptos::*;

#[component]
pub fn SelectorIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M8 9l4 -4l4 4"></path>
           <path d="M16 15l-4 4l-4 -4"></path>
        </svg>
    }
}
