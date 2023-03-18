use crate::gpt::image_params::ImageGenParams;
use crate::model::image::Image;
use crate::plugins::database::DbInstance;
use crate::utils::client::create_client;
use crate::utils::constants::CHATGPT_IMAGE_GENERATIONS_URL;
use crate::utils::error::Error;
use crate::utils::image_save::save_image;
use enki_shared::image_category::ImageCategory;
use enki_shared::image_format::ImageFormat;
use enki_shared::image_generate_record::ImageGenerateRecord;
use enki_shared::image_size::ImageSize;
use tauri::plugin::Plugin;
use tauri::{AppHandle, Invoke, Runtime, State};

#[tauri::command]
pub async fn generate_image<R: Runtime>(
    app: AppHandle<R>,
    db: State<'_, DbInstance>,
    prompt: String,
    n: usize,
    size: ImageSize,
    fmt: ImageFormat,
    category: ImageCategory,
    topic_id: i64,
) -> Result<Vec<String>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    print!("\n image generate start ... \n");

    let image_id = Image::create_image(
        pool,
        Some(prompt.clone()),
        None,
        None,
        category.to_string(),
        topic_id,
    )
    .await?;

    let client = create_client(pool).await?;

    let params = ImageGenParams {
        prompt,
        n,
        size: size.to_string(),
        response_format: fmt.to_string(),
    };

    let response_text = client
        .post(CHATGPT_IMAGE_GENERATIONS_URL)
        .json(&params)
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
pub async fn get_image_generate_record(
    db: State<'_, DbInstance>,
    id: i64,
) -> Result<Vec<ImageGenerateRecord>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    Image::retrieve_generate_record(pool, id).await
}

pub struct ImageGeneratePlugin<R: Runtime>(Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>);

impl<R: Runtime> ImageGeneratePlugin<R> {
    pub fn new() -> Self {
        Self(Box::new(tauri::generate_handler![
            generate_image,
            get_image_generate_record,
        ]))
    }
}

impl<R: Runtime> Plugin<R> for ImageGeneratePlugin<R> {
    fn name(&self) -> &'static str {
        "image_generate"
    }

    fn extend_api(&mut self, invoke: Invoke<R>) -> bool {
        self.0(invoke)
    }
}
