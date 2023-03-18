use crate::model::image_response::ImageResponse;
use crate::utils::config_path::app_config_dir;
use crate::utils::error::Error;
use base64::{engine::general_purpose, Engine as _};
use enki_shared::convert_src::convert_src;
use enki_shared::image::ImageData;
use enki_shared::image::Images;
use enki_shared::image_size::ImageSize;
use image::{guess_format, load_from_memory};
use sqlx::{Pool, Sqlite};
use std::fs::{create_dir_all, File};
use tauri::{AppHandle, Runtime};

pub async fn save_image<R: Runtime>(
    app: &AppHandle<R>,
    pool: &Pool<Sqlite>,
    size: ImageSize,
    image_id: i64,
    topic_id: i64,
    raw_text: String,
) -> Result<Vec<String>, Error> {
    let images = serde_json::from_str::<Images>(raw_text.as_str())?;

    match images {
        Images::Data { data } => {
            let mut result = Vec::new();

            for d in data.into_iter() {
                let uri = match d {
                    ImageData::Url { url } => url,
                    ImageData::Base64 { b64_json } => {
                        let bytes = general_purpose::STANDARD.decode(b64_json)?;

                        let img_fmt = guess_format(&bytes)?;
                        let img_unknown = load_from_memory(&bytes)?;
                        let img_ext = img_fmt.extensions_str().first().unwrap_or(&"png");

                        let config_dir = app_config_dir(&app)?;
                        let img_dir = config_dir.join("images");
                        create_dir_all(&img_dir)?;

                        let id = uuid::Uuid::new_v4().to_string();
                        let img_file = format!("{}.{}", id, img_ext);
                        let absolute_path = img_dir.join(img_file.as_str());

                        let mut output = File::create(&absolute_path)?;
                        img_unknown.write_to(&mut output, img_fmt)?;

                        convert_src(absolute_path.to_str().unwrap())
                    }
                };

                ImageResponse::create_image_response(
                    pool,
                    uri.as_str(),
                    size.to_string(),
                    image_id,
                    topic_id,
                )
                .await?;

                result.push(uri);
            }

            Ok(result)
        }
        Images::Error { error } => Err(Error::ImageGenerateFailed(error.message)),
    }
}
