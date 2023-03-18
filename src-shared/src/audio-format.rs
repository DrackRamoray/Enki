use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AudioFormat {
    TXT,
    SRT,
    VTT,
}

pub const AUDIO_FORMATS: &[(AudioFormat, &'static str)] = &[
    (AudioFormat::TXT, "text"),
    (AudioFormat::SRT, "srt"),
    (AudioFormat::VTT, "vtt"),
];

impl ToString for AudioFormat {
    fn to_string(&self) -> String {
        for &(ref fmt, name) in AUDIO_FORMATS.iter() {
            if self == fmt {
                return name.to_string();
            }
        }

        unreachable!();
    }
}

impl AudioFormat {
    pub fn vec() -> Vec<String> {
        AUDIO_FORMATS
            .iter()
            .map(|&(_, name)| name.to_string())
            .collect()
    }

    pub fn value_from_index(index: usize) -> Self {
        if index <= AUDIO_FORMATS.len() {
            return AUDIO_FORMATS[index].0;
        }

        panic!()
    }
}
