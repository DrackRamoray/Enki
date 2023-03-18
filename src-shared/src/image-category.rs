use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageCategory {
    Generate,
    Edit,
    Variate,
}

const IMAGE_CATEGORIES: &[(ImageCategory, &'static str)] = &[
    (ImageCategory::Generate, "generate"),
    (ImageCategory::Edit, "edit"),
    (ImageCategory::Variate, "variate"),
];

impl ToString for ImageCategory {
    fn to_string(&self) -> String {
        for (cate, name) in IMAGE_CATEGORIES.iter() {
            if cate == self {
                return name.to_string();
            }
        }

        unreachable!()
    }
}
