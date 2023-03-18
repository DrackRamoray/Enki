use crate::plugins::database::DbInstance;
use crate::utils::client::create_client;
use crate::utils::constants::CHATGPT_IMAGE_EDIT_URL;
use crate::utils::image_save::save_image;
use crate::{model::image::Image, utils::error::Error};
use enki_shared::convert_src::convert_src;
use enki_shared::image_edit_record::ImageEditRecord;
use enki_shared::{
    image_category::ImageCategory, image_format::ImageFormat, image_size::ImageSize,
};
use reqwest::multipart::{Form, Part};
use std::fs::read;
use tauri::{plugin::Plugin, AppHandle, Invoke, Runtime, State};

#[tauri::command]
async fn edit_image<R: Runtime>(
    app: AppHandle<R>,
    db: State<'_, DbInstance>,
    image: String,
    mask: Option<String>,
    prompt: String,
    n: usize,
    size: ImageSize,
    fmt: ImageFormat,
    category: ImageCategory,
    topic_id: i64,
) -> Result<Vec<String>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    println!("\n image edit start ... \n");

    let image_id = Image::create_image(
        pool,
        Some(prompt.clone()),
        Some(convert_src(image.as_str())),
        mask.clone().map(|m| convert_src(m.as_str())),
        category.to_string(),
        topic_id,
    )
    .await?;

    let client = create_client(pool).await?;

    let image_data = read(&image)?;

    let mut form = Form::new()
        .part("prompt", Part::text(prompt.clone()))
        .part("n", Part::text(n.to_string()))
        .part("size", Part::text(size.to_string()))
        .part("response_format", Part::text(fmt.to_string()))
        .part("image", Part::bytes(image_data).file_name(image));

    if let Some(m) = mask {
        let mask_data = read(&m)?;
        form = form.part("mask", Part::bytes(mask_data).file_name(m));
    }

    let response_text = client
        .post(CHATGPT_IMAGE_EDIT_URL)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    println!(
        "\n response finished: {:?} \n",
        &response_text[0..1000.min(response_text.len())]
    );

    save_image(&app, pool, size, image_id, topic_id, response_text).await
}

#[tauri::command]
pub async fn get_image_edit_record(
    db: State<'_, DbInstance>,
    id: i64,
) -> Result<Vec<ImageEditRecord>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    Image::retrieve_edit_record(pool, id).await
}

pub struct ImageEditPlugin<R: Runtime>(Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>);

impl<R: Runtime> ImageEditPlugin<R> {
    pub fn new() -> Self {
        Self(Box::new(tauri::generate_handler![
            edit_image,
            get_image_edit_record
        ]))
    }
}

impl<R: Runtime> Plugin<R> for ImageEditPlugin<R> {
    fn name(&self) -> &'static str {
        "image_edit"
    }

    fn extend_api(&mut self, invoke: Invoke<R>) -> bool {
        self.0(invoke)
    }
}
