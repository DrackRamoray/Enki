use enki_shared::{
    image_category::ImageCategory, image_format::ImageFormat, image_size::ImageSize,
    image_variate_record::ImageVariateRecord,
};
use reqwest::multipart::{Form, Part};
use tauri::{plugin::Plugin, AppHandle, Invoke, Runtime, State};

use crate::{
    model::image::Image,
    utils::{
        client::create_client, constants::CHATGPT_IMAGE_VARIATE_URL, error::Error,
        image_save::save_image,
    },
};
use enki_shared::convert_src::convert_src;

use super::database::DbInstance;

#[tauri::command]
async fn variate_image<R: Runtime>(
    app: AppHandle<R>,
    db: State<'_, DbInstance>,
    image: String,
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
        None,
        Some(convert_src(image.as_str())),
        None,
        category.to_string(),
        topic_id,
    )
    .await?;

    let client = create_client(pool).await?;

    let img_data = std::fs::read(&image)?;

    let form = Form::new()
        .text("n", n.to_string())
        .text("size", size.to_string())
        .text("response_format", fmt.to_string())
        .part("image", Part::bytes(img_data).file_name(image));

    let response_text = client
        .post(CHATGPT_IMAGE_VARIATE_URL)
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
pub async fn get_image_variate_record(
    db: State<'_, DbInstance>,
    id: i64,
) -> Result<Vec<ImageVariateRecord>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    Image::retrieve_variate_record(pool, id).await
}

pub struct ImageVariatePlugin<R: Runtime> {
    handler: Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>,
}

impl<R: Runtime> ImageVariatePlugin<R> {
    pub fn new() -> Self {
        Self {
            handler: Box::new(tauri::generate_handler![
                variate_image,
                get_image_variate_record
            ]),
        }
    }
}

impl<R: Runtime> Plugin<R> for ImageVariatePlugin<R> {
    fn name(&self) -> &'static str {
        "image_variate"
    }

    fn extend_api(&mut self, invoke: Invoke<R>) -> bool {
        (self.handler)(invoke)
    }
}
