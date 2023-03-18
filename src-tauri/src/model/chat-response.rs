use crate::utils::error::Error;
use enki_shared::chat::ChatMessage;
use futures::TryStreamExt;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

#[derive(sqlx::FromRow, Deserialize)]
pub struct ChatResponse {
    pub id: i64,
    pub role: String,
    pub raw: String,
    pub markdown: String,
    pub chat_id: i64,
    pub topic_id: i64,
}

impl ChatResponse {
    pub async fn retrieve_response_messages(
        pool: &Pool<Sqlite>,
        chat_id: i64,
        using_markdown: bool,
    ) -> Result<Vec<ChatMessage>, Error> {
        let mut result = Vec::new();
        let mut rows = sqlx::query_as::<_, ChatResponse>(
            "SELECT id, role, raw, markdown, chat_id, topic_id FROM chat_response WHERE chat_id = ? ORDER BY id"
        )
            .bind(chat_id)
            .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            result.push(ChatMessage {
                role: row.role,
                content: if using_markdown {
                    row.markdown
                } else {
                    row.raw
                },
            });
        }

        Ok(result)
    }

    pub async fn create_response(
        pool: &Pool<Sqlite>,
        role: &str,
        raw: &str,
        markdown: &str,
        chat_id: i64,
        topic_id: i64,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO chat_response (role, raw, markdown, chat_id, topic_id) VALUES (?, ?, ?, ?, ?)"
        )
            .bind(role)
            .bind(raw)
            .bind(markdown)
            .bind(chat_id)
            .bind(topic_id)
            .execute(pool)
            .await?;

        Ok(result.last_insert_rowid())
    }
}
