use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageSize {
    X256,
    X512,
    X1024,
}

pub const IMAGE_SIZES: &[(ImageSize, &'static str)] = &[
    (ImageSize::X256, "256x256"),
    (ImageSize::X512, "512x512"),
    (ImageSize::X1024, "1024x1024"),
];

impl ToString for ImageSize {
    fn to_string(&self) -> String {
        for &(ref size, name) in IMAGE_SIZES.iter() {
            if self == size {
                return name.to_string();
            }
        }

        unreachable!();
    }
}

impl ImageSize {
    pub fn vec() -> Vec<String> {
        IMAGE_SIZES
            .iter()
            .map(|&(_, name)| name.to_string())
            .collect()
    }

    pub fn value_from_index(index: usize) -> Self {
        if index <= IMAGE_SIZES.len() {
            return IMAGE_SIZES[index].0;
        }

        panic!()
    }
}
