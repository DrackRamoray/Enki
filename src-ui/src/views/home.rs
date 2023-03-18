use crate::assists::error::beautify_error;
use crate::assists::notify::{notify, NotifyType};
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let navigate_to = create_action(cx, move |params: &usize| {
        let index = *params;
        let navigate = use_navigate(cx);

        async move {
            if let Err(err) = navigate(
                &format!("/topic/create?topic={}", index),
                Default::default(),
            ) {
                notify(cx, NotifyType::Error, beautify_error(err));
            }
        }
    });

    view! {
        cx,
        <div class="home__nav">
            <div class="home__nav-container">
                <For
                    each=move || { TopicCategory::vec().into_iter().enumerate() }
                    key=|(index, _)| *index
                    view=move |cx, (index, item)| view! {
                        cx,
                        <div class="home__nav-item">
                            <div class="home__nav-item--hex">
                                <div></div>
                                <div></div>
                                <div></div>
                            </div>
                            <div class="home__nav-item--hex">
                                <div></div>
                                <div></div>
                                <div></div>
                            </div>
                            <a href="#" class="home__nav-item__content" on:click=move |_| navigate_to.dispatch(index) >
                                <span class="home__nav-item__content__inner">
                                    <span class="home__nav-item__title">{ item.0 }</span>
                                </span>
                                <svg
                                    viewBox="0 0 173.20508075688772 200"
                                    height="200"
                                    width="174"
                                    version="1.1"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path d="M86.60254037844386 0L173.20508075688772 50L173.20508075688772 150L86.60254037844386 200L0 150L0 50Z"></path>
                                </svg>
                            </a>
                        </div>
                    }
                />
            </div>
        </div>
    }
}
