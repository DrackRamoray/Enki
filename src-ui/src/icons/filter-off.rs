use leptos::*;

#[component]
pub fn FilterOffIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M3 3l18 18"></path>
           <path d="M9 5h9.5a1 1 0 0 1 .5 1.5l-4.049 4.454m-.951 3.046v5l-4 -3v-4l-5 -5.5a1 1 0 0 1 .18 -1.316"></path>
        </svg>
    }
}
