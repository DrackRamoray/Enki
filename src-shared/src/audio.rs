use serde::{Deserialize, Serialize};

use crate::{audio_format::AudioFormat, audio_model::AudioModel};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioParams {
    pub file: String,
    pub model: AudioModel,
    pub prompt: String,
    pub fmt: AudioFormat,
    pub temperature: f64,
    pub language: Option<String>,
    pub topic_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct AudioRecordParams {
    pub id: i64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AudioData {
    pub id: i64,
    pub file: String,
    pub prompt: String,
    pub response: String,
}
