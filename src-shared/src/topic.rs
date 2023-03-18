use crate::topic_category::TopicCategory;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub category: TopicCategory,
}

#[derive(Debug, Serialize)]
pub struct NewTopic {
    pub title: String,
    pub category: TopicCategory,
}

#[derive(Debug, Serialize)]
pub struct EditTopic {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct RemoveTopic {
    pub id: i64,
}

#[derive(Debug, Serialize)]
pub struct SearchTopicList {
    pub title: Option<String>,
    pub category: Option<TopicCategory>,
}

#[derive(Debug, Serialize)]
pub struct FetchTopic {
    pub id: i64,
}
