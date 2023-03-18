use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageData {
    Url { url: String },
    Base64 { b64_json: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Images {
    Data { data: Vec<ImageData> },
    Error { error: ImageErrorMsg },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageErrorMsg {
    pub message: String,
}
