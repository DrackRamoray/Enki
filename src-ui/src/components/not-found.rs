use crate::assists::location::location_to_home;
use crate::icons::home_up::*;
use crate::icons::not_found::*;
use leptos::*;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="h-full flex justify-center items-center flex-col dark:text-dark-quinary">
            <div> <NotFoundIcon /> </div>
            <div>
                <button class="btn" on:click=move |_| location_to_home(cx) >
                    <HomeUpIcon />
                </button>
            </div>
        </div>
    }
}
