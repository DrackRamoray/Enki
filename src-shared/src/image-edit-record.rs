use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageEditRecord {
    pub image: String,
    pub mask: Option<String>,
    pub prompt: String,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageEditRecordParams {
    pub id: i64,
}
