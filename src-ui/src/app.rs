use crate::assists::location::{location_to_home, location_to_topic_create};
use crate::components::auth::*;
use crate::components::mask::*;
use crate::components::not_found::*;
use crate::components::proxy::*;
use crate::icons::home::*;
use crate::icons::menu::*;
use crate::icons::plus::*;
use crate::stores::app_ctx::use_app_ctx;
use crate::views::audio_transcript::*;
use crate::views::audio_translate::*;
use crate::views::chat::*;
use crate::views::home::*;
use crate::views::img_edit::*;
use crate::views::img_gen::*;
use crate::views::img_var::*;
use crate::views::sidebar::*;
use crate::views::topic_create::*;
use enki_shared::topic_category::TopicCategory;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(cx, true);
    let visible = Signal::derive(cx, move || !hidden.get());
    let toggle = move |_| set_hidden.update(|value| *value = !*value);

    let ctx = use_app_ctx(cx);
    provide_context(cx, ctx);

    window_event_listener("unhandledrejection", move |evt| {
        error!("{:?}", evt);
        ctx.set_sending.set(false);
        evt.stop_propagation();
    });

    let go_to = move |_| location_to_topic_create(cx);
    let go_home = move |_| location_to_home(cx);

    view! {
        cx,
        <header class="header">
            <div class="header__toggle-btn">
                <button class="btn" on:click=toggle><MenuIcon /></button>
                <button class="header__topic--add btn" on:click=go_to><PlusIcon /></button>
                <button class="btn" on:click=go_home><HomeIcon /></button>
            </div>
            <div class="header__btn-group">
                <Proxy />
                <Auth />
            </div>
        </header>
        <aside class="aside" class:visible=visible>
            <Sidebar />
        </aside>
        <main class="main">
            <Routes>
                <Route path="/" view=move |cx| view!{ cx, <Home /> } />
                <Route path="/topic/create" view=move |cx| view! { cx, <TopicCreate /> } />
                <Route path=TopicCategory::Chat.to_path() view=move |cx| ctx.active_topic_id.with(|&id| match id {
                    0 => view!{ cx, <> <NotFound /> </> },
                    _ => view!{ cx, <> <Chat /> </> },
                })/>
                <Route path=TopicCategory::AudioTranscript.to_path() view=move |cx| ctx.active_topic_id.with(|&id| match id {
                    0 => view!{ cx, <> <NotFound /> </> },
                    _ => view!{ cx, <> <AudioTranscript /> </> },
                })/>
                <Route path=TopicCategory::AudioTranslate.to_path() view=move |cx| ctx.active_topic_id.with(|&id| match id {
                    0 => view!{ cx, <> <NotFound /> </> },
                    _ => view!{ cx, <> <AudioTranslate /> </> },
                })/>
                <Route path=TopicCategory::ImageGenerate.to_path() view=move |cx| ctx.active_topic_id.with(|&id| match id {
                    0 => view!{ cx, <> <NotFound /> </> },
                    _ => view!{ cx, <> <ImgGen /> </> },
                })/>
                <Route path=TopicCategory::ImageEdit.to_path() view=move |cx| ctx.active_topic_id.with(|&id| match id {
                    0 => view!{ cx, <> <NotFound /> </> },
                    _ => view!{ cx, <> <ImgEdit /> </> },
                })/>
                <Route path=TopicCategory::ImageVariate.to_path() view=move |cx| ctx.active_topic_id.with(|&id| match id {
                    0 => view!{ cx, <> <NotFound /> </> },
                    _ => view!{ cx, <> <ImgVar /> </> },
                })/>
            </Routes>
        </main>
        <Mask hidden=hidden set_hidden=set_hidden class="mask" />
    }
}
