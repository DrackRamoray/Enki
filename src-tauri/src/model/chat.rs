use crate::model::chat_response::ChatResponse;
use crate::utils::error::Error;
use enki_shared::chat::ChatMessage;
use enki_shared::chat_record::ChatRecord;
use futures::TryStreamExt;
use sqlx::{Pool, Sqlite};

#[derive(sqlx::FromRow, serde::Deserialize)]
pub struct Chat {
    pub id: i64,
    pub role: String,
    pub message: String,
    pub topic_id: i64,
}

impl Chat {
    pub async fn retrieve_messages(
        pool: &Pool<Sqlite>,
        topic_id: i64,
    ) -> Result<Vec<ChatMessage>, Error> {
        let mut messages = Vec::new();
        let mut rows = sqlx::query_as::<_, Chat>(
            "SELECT id, role, message, topic_id FROM chat WHERE topic_id = ? ORDER BY id",
        )
        .bind(topic_id)
        .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            messages.push(ChatMessage {
                role: row.role,
                content: row.message,
            });
            let responses = ChatResponse::retrieve_response_messages(pool, row.id, false).await?;
            messages.extend(responses);
        }

        Ok(messages)
    }

    pub async fn retrieve_records(
        pool: &Pool<Sqlite>,
        topic_id: i64,
    ) -> Result<Vec<ChatRecord>, Error> {
        let mut records = Vec::new();
        let mut rows = sqlx::query_as::<_, Chat>(
            "SELECT id, role, message, topic_id FROM chat WHERE topic_id = ? ORDER BY id",
        )
        .bind(topic_id)
        .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            let responses = ChatResponse::retrieve_response_messages(pool, row.id, true).await?;
            records.push(ChatRecord {
                speaker: ChatMessage {
                    role: row.role,
                    content: row.message,
                },
                responder: responses,
            });
        }

        Ok(records)
    }

    pub async fn create_chat(
        pool: &Pool<Sqlite>,
        role: &str,
        message: &str,
        topic_id: i64,
    ) -> Result<i64, Error> {
        let result = sqlx::query("INSERT INTO chat (role, message, topic_id) VALUES (?, ?, ?)")
            .bind(role)
            .bind(message)
            .bind(topic_id)
            .execute(pool)
            .await?;

        Ok(result.last_insert_rowid())
    }
}
