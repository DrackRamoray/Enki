use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TopicCategory {
    Chat,
    ImageGenerate,
    ImageEdit,
    ImageVariate,
    AudioTranscript,
    AudioTranslate,
}

pub type TopicCategoryString = (TopicCategory, &'static str, &'static str);

pub const TOPIC_CATEGORIES: &[TopicCategoryString] = &[
    (TopicCategory::Chat, "chat", "/chat"),
    (
        TopicCategory::AudioTranscript,
        "audio transcript",
        "/audio/transcript",
    ),
    (
        TopicCategory::AudioTranslate,
        "audio translate",
        "/audio/translate",
    ),
    (
        TopicCategory::ImageGenerate,
        "image generate",
        "/image/generate",
    ),
    (TopicCategory::ImageEdit, "image edit", "/image/edit"),
    (
        TopicCategory::ImageVariate,
        "image variate",
        "/image/variate",
    ),
];

impl ToString for TopicCategory {
    fn to_string(&self) -> String {
        for &(ref topic, name, _path) in TOPIC_CATEGORIES.iter() {
            if topic == self {
                return name.to_string();
            }
        }

        unreachable!();
    }
}

impl TopicCategory {
    pub fn to_path(&self) -> String {
        for &(ref topic, _name, path) in TOPIC_CATEGORIES.iter() {
            if topic == self {
                return path.to_string();
            }
        }

        unreachable!();
    }

    pub fn to_index(&self) -> i32 {
        for (index, &(ref topic, _name, _path)) in TOPIC_CATEGORIES.iter().enumerate() {
            if topic == self {
                return index as i32;
            }
        }

        -1
    }

    pub fn vec() -> Vec<(String, String)> {
        TOPIC_CATEGORIES
            .iter()
            .map(|&(_, name, path)| (name.to_owned(), path.to_owned()))
            .collect()
    }

    pub fn names() -> Vec<String> {
        TOPIC_CATEGORIES
            .iter()
            .map(|&(_, name, _path)| name.to_owned())
            .collect()
    }

    pub fn name_to_value(keyword: &str) -> Self {
        for &(topic, name, _path) in TOPIC_CATEGORIES.iter() {
            if name == keyword {
                return topic;
            }
        }

        panic!();
    }

    pub fn value_from_index(index: usize) -> Self {
        if index <= TOPIC_CATEGORIES.len() {
            return TOPIC_CATEGORIES[index].0;
        }

        panic!()
    }
}
