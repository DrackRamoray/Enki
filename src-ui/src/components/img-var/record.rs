use crate::components::image_list::{render_image_list, render_image_list_error};
use crate::components::img_var_speaker::*;
use enki_shared::image_variate_record::ImageVariateRecord;
use leptos::*;

#[component]
pub fn ImgVarRecord(cx: Scope, data: Vec<ImageVariateRecord>) -> impl IntoView {
    view! {
        cx,
        <For
            each=move || data.clone().into_iter().enumerate()
            key=|(index,_)| *index
            view=move |cx, (_, item)| {
                let image = item.image;
                let images = item.images;
                let exists = images.len() > 0;

                view! {
                    cx,
                    <div>
                        <ImgVarPromptSpeaker image=image auto_scroll=false />
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
