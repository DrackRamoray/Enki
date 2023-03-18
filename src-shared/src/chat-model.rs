use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatModel {
    #[serde(rename = "gpt-3.5-turbo")]
    GPT,
    #[serde(rename = "gpt-3.5-turbo-0301")]
    GPT0301,
}

pub const CHAT_MODELS: &[(ChatModel, &'static str)] = &[
    (ChatModel::GPT, "gpt-3.5-turbo"),
    (ChatModel::GPT0301, "gpt-3.5-turbo-0301"),
];

impl ToString for ChatModel {
    fn to_string(&self) -> String {
        for &(ref model, name) in CHAT_MODELS.iter() {
            if model == self {
                return name.to_string();
            }
        }

        unreachable!();
    }
}

impl ChatModel {
    pub fn vec() -> Vec<String> {
        CHAT_MODELS
            .iter()
            .map(|&(_, name)| name.to_string())
            .collect()
    }

    pub fn value_from_index(index: usize) -> Self {
        if index <= CHAT_MODELS.len() {
            return CHAT_MODELS[index].0;
        }

        panic!()
    }
}
