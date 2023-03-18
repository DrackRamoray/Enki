use crate::components::image_list::{render_image_list, render_image_list_error};
use crate::components::prompt_speaker::render_prompt_speaker;
use enki_shared::image_generate_record::ImageGenerateRecord;
use leptos::html::Ul;
use leptos::*;

#[component]
pub fn ImgGenRecord(cx: Scope, data: Vec<ImageGenerateRecord>) -> impl IntoView {
    view! {
        cx,
        <For
            each=move || data.clone().into_iter().enumerate()
            key=|(index,_)| *index
            view=move |cx, (_, item)| {
                let prompt = item.prompt;
                let images = item.images;
                let exists = images.len() > 0;

                view! {
                    cx,
                    <div>
                        {
                            let node_ref: NodeRef<Ul> = create_node_ref(cx);

                            move || render_prompt_speaker(cx, node_ref, prompt.clone())
                        }
                        {
                            move || match exists {
                                false => render_image_list_error(cx, format!("")),
                                true => render_image_list(cx, images.clone(), false),
                            }
                        }
                    </div>
                }
            }
        />
    }
}
