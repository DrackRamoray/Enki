use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImageGenParams {
    pub prompt: String,
    pub n: usize,
    pub size: String,
    pub response_format: String,
}
