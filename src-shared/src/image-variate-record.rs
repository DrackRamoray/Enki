use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageVariateRecord {
    pub image: String,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageVariateRecordParams {
    pub id: i64,
}
