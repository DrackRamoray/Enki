use leptos::html::Ul;
use leptos::*;

pub fn render_prompt_speaker(
    cx: Scope,
    node_ref: NodeRef<Ul>,
    message: impl IntoView,
) -> HtmlElement<Ul> {
    view! {
        cx,
        <ul class="prompt__speaker" node_ref=node_ref >
            <li class="prompt__speaker__message">{ message }</li>
            <li class="p-1">
                <img src="/public/avatar.png" />
            </li>
        </ul>
    }
}
