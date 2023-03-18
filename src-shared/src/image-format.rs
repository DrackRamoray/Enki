use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageFormat {
    Url,
    Base64,
}

pub const IMAGE_FORMATS: &[(ImageFormat, &'static str)] =
    &[(ImageFormat::Url, "url"), (ImageFormat::Base64, "b64_json")];

impl ToString for ImageFormat {
    fn to_string(&self) -> String {
        for &(ref size, name) in IMAGE_FORMATS.iter() {
            if self == size {
                return name.to_string();
            }
        }

        unreachable!();
    }
}

impl ImageFormat {
    pub fn vec() -> Vec<String> {
        IMAGE_FORMATS
            .iter()
            .map(|&(_, name)| name.to_string())
            .collect()
    }

    pub fn value_from_index(index: usize) -> Self {
        if index <= IMAGE_FORMATS.len() {
            return IMAGE_FORMATS[index].0;
        }

        panic!()
    }
}
