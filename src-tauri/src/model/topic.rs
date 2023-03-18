use enki_shared::topic::Topic as TopicData;
use enki_shared::topic_category::TopicCategory;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Topic {
    id: i64,
    title: String,
    category: String,
    #[allow(dead_code)]
    status: i32,
}

impl Topic {
    pub fn into_topic(self) -> TopicData {
        TopicData {
            id: self.id,
            title: self.title,
            category: TopicCategory::name_to_value(self.category.as_str()),
        }
    }
}
