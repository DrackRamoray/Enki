use crate::image_category::ImageCategory;
use crate::{image_format::ImageFormat, image_size::ImageSize};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageVariateParams {
    pub image: String,
    pub n: usize,
    pub size: ImageSize,
    pub fmt: ImageFormat,
    pub category: ImageCategory,
    pub topic_id: i64,
}
