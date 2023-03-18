use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AudioModel {
    Whisper1,
}

pub const AUDIO_MODELS: &[(AudioModel, &'static str)] = &[(AudioModel::Whisper1, "whisper-1")];

impl ToString for AudioModel {
    fn to_string(&self) -> String {
        for &(ref fmt, name) in AUDIO_MODELS.iter() {
            if self == fmt {
                return name.to_string();
            }
        }

        unreachable!();
    }
}

impl AudioModel {
    pub fn vec() -> Vec<String> {
        AUDIO_MODELS
            .iter()
            .map(|&(_, name)| name.to_string())
            .collect()
    }

    pub fn value_from_index(index: usize) -> Self {
        if index <= AUDIO_MODELS.len() {
            return AUDIO_MODELS[index].0;
        }

        panic!()
    }
}
