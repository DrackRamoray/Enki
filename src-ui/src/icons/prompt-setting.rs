use leptos::*;

#[component]
pub fn PromptSettingIcon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
           <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
           <path d="M11.996 19.98a9.868 9.868 0 0 1 -4.296 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.842 1.572 2.783 3.691 2.77 5.821"></path>
           <path d="M19.001 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path>
           <path d="M19.001 15.5v1.5"></path>
           <path d="M19.001 21v1.5"></path>
           <path d="M22.032 17.25l-1.299 .75"></path>
           <path d="M17.27 20l-1.3 .75"></path>
           <path d="M15.97 17.25l1.3 .75"></path>
           <path d="M20.733 20l1.3 .75"></path>
        </svg>
    }
}
