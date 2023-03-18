use crate::assists::scroll::scroll_to_bottom;
use enki_shared::convert_src::convert_src;
use leptos::*;

#[component]
pub fn AudioSpeaker(cx: Scope, file: String, prompt: String, auto_scroll: bool) -> impl IntoView {
    let node_ref = create_node_ref(cx);

    if auto_scroll {
        scroll_to_bottom(node_ref);
    }

    view! {
        cx,
        <ul class="prompt__speaker" node_ref=node_ref>
            <li class="prompt__speaker__message">
                <div class="prompt__speaker__audio">
                    <audio prop:src=move || convert_src(file.as_str())  controls ></audio>
                </div>
                <span class="float-right">{ prompt }</span>
            </li>
            <li class="p-1">
                <img src="/public/avatar.png" />
            </li>
        </ul>
    }
}
