use leptos::*;

#[component]
pub fn LoadingIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg class="animate-spin" xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M12 6l0 -3"></path>
           <path d="M16.25 7.75l2.15 -2.15"></path>
           <path d="M18 12l3 0"></path>
           <path d="M16.25 16.25l2.15 2.15"></path>
           <path d="M12 18l0 3"></path>
           <path d="M7.75 16.25l-2.15 2.15"></path>
           <path d="M6 12l-3 0"></path>
           <path d="M7.75 7.75l-2.15 -2.15"></path>
        </svg>
    }
}
