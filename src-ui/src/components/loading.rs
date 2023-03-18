use crate::icons::loading::*;
use leptos::*;

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="absolute z-[40] w-full h-full top-0 left-0 flex justify-center items-center opacity-50 dark:bg-dark-primary">
            <div class="text-gray-400">
                <LoadingIcon />
            </div>
        </div>
    }
}
