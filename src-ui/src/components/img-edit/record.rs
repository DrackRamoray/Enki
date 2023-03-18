use crate::components::image_list::{render_image_list, render_image_list_error};
use crate::components::img_edit_speaker::*;
use enki_shared::image_edit_record::ImageEditRecord;
use leptos::*;

#[component]
pub fn ImgEditRecord(cx: Scope, data: Vec<ImageEditRecord>) -> impl IntoView {
    view! {
        cx,
        <For
            each=move || data.clone().into_iter().enumerate()
            key=|(index,_)| *index
            view=move |cx, (_, item)| {
                let image = item.image;
                let mask = item.mask.unwrap_or_default();
                let prompt = item.prompt;
                let images = item.images;
                let exists = images.len() > 0;

                view! {
                    cx,
                    <div>
                        <ImgEditPromptSpeaker image=image mask=mask prompt=prompt auto_scroll=false />
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
