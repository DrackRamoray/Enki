use crate::image_category::ImageCategory;
use crate::image_format::ImageFormat;
use crate::image_size::ImageSize;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerateParams {
    pub prompt: String,
    pub n: usize,
    pub size: ImageSize,
    pub fmt: ImageFormat,
    pub category: ImageCategory,
    pub topic_id: i64,
}
