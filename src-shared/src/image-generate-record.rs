use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageGenerateRecord {
    pub prompt: String,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageGenerateRecordParams {
    pub id: i64,
}
